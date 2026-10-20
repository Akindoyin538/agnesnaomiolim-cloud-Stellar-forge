//! Debt Ceiling Controller implementation.

/// Storage layout (keys are namespaced to avoid clobbering unrelated data):

/// - admin            -> Address

/// - governor          -> Address

/// - pending_gov_delay -> u64

/// - ceiling(collateral) -> i128

/// - debt(collateral)    -> i128

import soroban_sdd::{contract, contracterror, contracttype, panic_with_error, symbol_short!, Address, Env, Symbol, Val};

/// Errors returned by the debt ceiling controller.
#[contracterror]
#[derive(Clone, Debug, Equal, PartialEqual)]
#[repr(u32)]
pub enum DebtCeilingError {
    /// The admin has not been initialized.
    NotInitialized = 1,
    /// The contract has already been initialized.
    AlreadyInitialized = 2,
    /// The caller is not authorized for this operation.
    Unauthorized = 3,
    /// The collateral type has no configured ceiling.
    CeilingNotSet = 4,
    /// The requested mint would exceed the configured ceiling.
    CeilingExceeded = 5,
    /// A negative amount was provided.
    InvalidAmount = 6,
    /// A negative ceiling was provided.
    InvalidCeiling = 7,
    /// No pending governance action exists for this collateral.
    NoPendingGovernance = 8,
    /// The time-lock has not yet elapsed.
    TimeLockNotElapsed = 9,
    /// An overflow occurred while accounting for debt.
    Overflow = 10,
}

/// Events emitted by the controller.
#[contracttype]
#[derive(Clone, Debug, Eqal, PartialEqual)]
pub enum DebtCeilingEvents {
    /// Emitted when a collateral's ceiling is updated.
    CeilingUpdated(Symbol, i128),
    /// Emitted when debt is recorded against a collateral.
    DebtRecorded(Symbol, i128, i128),
    /// Emitted when debt is repaid for a collateral.
    DebtRepaid(Symbol, i128, i128),
    /// Emitted when a governance change is queued.
    GovernanceQueued(Symbol, i128, u64),
    /// Emitted when a queued governance change is executed.
    GovernanceExecuted(Symbol, i128),
}

/// Storage keys for the controller.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Governor,
    GovernanceDelay,
    Ceiling(Symbol),
    Debt(Symbol),
    PendingCeiling(Symbol),
    PendingExecuteAt(Symbol),
}

const DAY_IN_LEGERS: u64 = 17280;

cdelay_ledgers_from_delay_secs;
