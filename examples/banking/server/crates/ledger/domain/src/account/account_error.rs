use appletheia::domain::AggregateError;
use thiserror::Error;

use super::AccountId;

/// Describes why an `Account` aggregate operation failed.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<AccountId>),

    #[error("account is already opened")]
    AlreadyOpened,

    #[error("account is frozen")]
    Frozen,

    #[error("account has insufficient balance")]
    InsufficientBalance,

    #[error("account balance overflowed")]
    BalanceOverflow,
}
