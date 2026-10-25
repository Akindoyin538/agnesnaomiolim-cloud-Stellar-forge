// Copyright 2024 Stellar-K8s Contributors
// SPDX-License-Identifier: Apache-2.0
//! Stellar-K8s controller crate.
//!
//! This crate provides the deployment pipeline and admission webhook modules
//! used by the Stellar-K8s operator.  It is intentionally kept lean so it can
//! be compiled quickly and linked into the operator binary without pulling in
//! heavy optional dependencies.
//!
//! # Modules
//!
//! - [`apis`] — Custom resource validation and API schema utilities.
//! - [`deployment`] — StellarNode deployment pipeline, including the
//!   [`deployment::optimizer`] WASM bytecode optimizer.
//! - [`quorum`] — Quorum-set graph analysis utilities.
//! - [`webhook`] — Admission webhook handlers, including the
//!   [`webhook::wasm_mutator`] MutatingAdmissionWebhook for WASM optimization.
//! - [`ingestion`] — Asynchronous XDR ledger ring buffer and stream manager
//!   (issue #75).  Decouples Stellar Core frame ingestion from the operator
//!   reconciliation loop via a bounded ring buffer with configurable
//!   backpressure and a parallel producer-consumer worker pool.
//! - [`crd`] — Custom resources owned by this crate, including the
//!   [`crd::testnet::StellarTestnet`] CRD for ephemeral testnets (issue #246).
//! - [`testnet`] — Reconciliation loop that provisions and tears down
//!   ephemeral single-node Stellar networks from `StellarTestnet` resources.

pub mod apis;
pub mod crd;
pub mod deployment;
pub mod ingestion;
pub mod quorum;
pub mod testnet;
pub mod webhook;

/// Standard test logger initialization macro helper.
///
/// Tests emitting raw tracing logs without initialization can output messy warnings.
/// This macro initializes a test subscriber (or defaults to a silent/noop subscriber)
/// ensuring clean test output. It uses `std::sync::Once` so multiple tests calling it
/// concurrently or sequentially within the same test process only initialize once.
#[macro_export]
macro_rules! init_test_logger {
    () => {{
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = tracing::subscriber::set_global_default(
                tracing::subscriber::NoSubscriber::default(),
            );
        });
    }};
    ($subscriber:expr) => {{
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = tracing::subscriber::set_global_default($subscriber);
        });
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_test_logger_macro() {
        init_test_logger!();
        // Emitting tracing events after initialization should succeed cleanly
        tracing::info!("test logger initialized cleanly");
        tracing::debug!("debug message in test");
    }

    #[test]
    fn test_init_test_logger_idempotent() {
        init_test_logger!();
        init_test_logger!();
        tracing::warn!("test multiple initializations");
    }
}
