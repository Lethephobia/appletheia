use thiserror::Error;

/// Describes why an `AccountBalance` operation failed.
#[derive(Debug, Error)]
pub enum AccountBalanceError {
    #[error("account balance overflowed")]
    BalanceOverflow,

    #[error("account has insufficient balance")]
    InsufficientBalance,
}
