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
//! - [`deployment`] — StellarNode deployment pipeline, including the
//!   [`deployment::optimizer`] WASM bytecode optimizer.
//! - [`quorum`] — Quorum-set graph analysis utilities.
//! - [`wasm`] — Warm WASM execution sandbox pooling, including the
//!   [`wasm::sandbox_pool`] concurrent object pool and the
//!   [`wasm::memory_scrubber`] linear-memory sanitization routines.
//! - [`webhook`] — Admission webhook handlers, including the
//!   [`webhook::wasm_mutator`] MutatingAdmissionWebhook for WASM optimization.

pub mod deployment;
pub mod quorum;
pub mod wasm;
pub mod webhook;

pub use wasm::sandbox_pool::{
    PoolConfig, PoolStats, SandboxPool, SandboxPoolError, WarmSandbox,
};
pub use wasm::memory_scrubber::{MemoryScrubber, ScrubError, ScrubReport};