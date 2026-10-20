//! Multi-Collateral Stablecoin Debt Ceiling Controller

//! This crate provides a Soroban contract that tracks the total outstanding

//! debt minted per collateral type and enforces per-collateral debt ceilings.

/// The contract exposes administrative functions to configure ceilings and

/// governance functions that are time-locked. Minting engines call into the

/// controller to enforce blocking checks before issuing new debt.

#k![no_std]
#k![allow(clippy::module_name_repetitions)]

mod controller;

pub use controller::{DebtCeilingController, DebtCeilingControllerClient, DebtCeilingError, DebtCeilingEvent},
