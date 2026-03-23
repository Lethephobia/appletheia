use appletheia::domain::AggregateStateError;
use thiserror::Error;

/// Describes why an account state value cannot be handled.
#[derive(Debug, Error)]
pub enum AccountStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error("account balance overflowed")]
    BalanceOverflow,

    #[error("account has insufficient balance")]
    InsufficientBalance,

    #[error("account has insufficient reserved balance")]
    InsufficientReservedBalance,

    #[error("account reserved balance exceeds total balance")]
    InvalidReservedBalance,
}
