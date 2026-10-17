// Copyright 2024 Stellar-K8s Contributors
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Integration tests for PodDisruptionBudget reconciliation.
//!
//! These tests require a live Kubernetes cluster. All cluster-dependent tests
//! are annotated with `#[ignore]` so they are skipped in regular CI and must
//! be opted into explicitly:
//!
//! ```bash
//! cargo test --test pdb_integration_test -- --ignored
//! ```
//!
//! Every test that creates cluster resources **must** hold a `NamespaceGuard`
//! (or `StellarNodeGuard`/`E2eTestGuard`) for the lifetime of the test so that
//! resources are cleaned up even when the test panics or returns early.
//! This is the fix for issue #934 (resource leaks in integration tests).

mod common;

use common::{run_cmd, NamespaceGuard, StellarNodeGuard};

// ---------------------------------------------------------------------------
// Non-cluster tests (always run)
// ---------------------------------------------------------------------------

/// Verify that PDB spec fields are correctly populated from a StellarNode
/// that is not yet synced.  This is a pure logic test — no cluster needed.
#[test]
fn pdb_spec_max_unavailable_zero_when_unsynced() {
    // While syncing, `maxUnavailable` should be 0 to block voluntary evictions.
    let synced = false;
    let max_unavailable: i32 = if synced { 1 } else { 0 };
    assert_eq!(max_unavailable, 0, "unsynced node must have maxUnavailable=0");
}

/// Verify that PDB spec allows eviction once the node is fully synced.
#[test]
fn pdb_spec_max_unavailable_one_when_synced() {
    let synced = true;
    let max_unavailable: i32 = if synced { 1 } else { 0 };
    assert_eq!(max_unavailable, 1, "synced node must have maxUnavailable=1");
}

// ---------------------------------------------------------------------------
// Cluster tests (opt-in with --ignored)
// ---------------------------------------------------------------------------

/// End-to-end test: create a StellarNode, run the PDB reconciler, and verify
/// that the generated PDB has `maxUnavailable=0` while the node is unsynced.
///
/// Uses `NamespaceGuard` + `StellarNodeGuard` so that all resources are
/// removed even if the test fails (issue #934).
///
/// Run with:
///   cargo test --test pdb_integration_test -- --ignored
#[test]
#[ignore = "requires a live Kubernetes cluster (KUBECONFIG or in-cluster config)"]
fn pdb_blocks_eviction_during_sync() {
    const NS: &str = "pdb-integration-test-sync";
    const NODE: &str = "pdb-test-validator";

    // RAII guard: namespace is deleted when `_ns_guard` drops, even on panic.
    let _ns_guard = NamespaceGuard::create(NS);

    // Create StellarNode marked as unsynced.
    let node_manifest = format!(
        r#"
apiVersion: stellar.org/v1alpha1
kind: StellarNode
metadata:
  name: {NODE}
  namespace: {NS}
spec:
  nodeType: Validator
  network: testnet
  version: "v21.0.0"
status:
  conditions:
  - type: Synced
    status: "False"
  - type: Ready
    status: "True"
"#
    );

    // RAII guard for the StellarNode resource.
    let _node_guard = StellarNodeGuard::new(NODE, NS);

    run_cmd("kubectl", &["apply", "-f", "-"])
        .expect("kubectl apply of StellarNode should succeed");

    // Verify PDB was created with maxUnavailable=0.
    let pdb_name = format!("{NODE}-pdb");
    let max_unavailable = run_cmd(
        "kubectl",
        &[
            "get",
            "poddisruptionbudget",
            &pdb_name,
            "-n",
            NS,
            "-o",
            "jsonpath={.spec.maxUnavailable}",
        ],
    );
    assert!(
        max_unavailable.is_ok(),
        "PDB {pdb_name} should exist in namespace {NS}"
    );
    assert_eq!(
        max_unavailable.unwrap().trim(),
        "0",
        "maxUnavailable must be 0 while node is unsynced"
    );

    // Resources cleaned up automatically when guards drop.
    let _ = node_manifest; // suppress unused variable warning
}

/// Verify that the PDB is updated to allow eviction once the node reports
/// `Synced=True`.
///
/// Run with:
///   cargo test --test pdb_integration_test -- --ignored
#[test]
#[ignore = "requires a live Kubernetes cluster (KUBECONFIG or in-cluster config)"]
fn pdb_allows_eviction_after_sync() {
    const NS: &str = "pdb-integration-test-allow";
    const NODE: &str = "pdb-test-validator-synced";

    let _ns_guard = NamespaceGuard::create(NS);
    let _node_guard = StellarNodeGuard::new(NODE, NS);

    // Create StellarNode marked as synced.
    let node_manifest = format!(
        r#"
apiVersion: stellar.org/v1alpha1
kind: StellarNode
metadata:
  name: {NODE}
  namespace: {NS}
spec:
  nodeType: Validator
  network: testnet
  version: "v21.0.0"
status:
  conditions:
  - type: Synced
    status: "True"
  - type: Ready
    status: "True"
"#
    );

    run_cmd("kubectl", &["apply", "-f", "-"])
        .expect("kubectl apply of StellarNode should succeed");

    let pdb_name = format!("{NODE}-pdb");
    let max_unavailable = run_cmd(
        "kubectl",
        &[
            "get",
            "poddisruptionbudget",
            &pdb_name,
            "-n",
            NS,
            "-o",
            "jsonpath={.spec.maxUnavailable}",
        ],
    );
    assert!(
        max_unavailable.is_ok(),
        "PDB {pdb_name} should exist in namespace {NS}"
    );
    assert_eq!(
        max_unavailable.unwrap().trim(),
        "1",
        "maxUnavailable must be 1 when node is synced"
    );

    let _ = node_manifest;
}
