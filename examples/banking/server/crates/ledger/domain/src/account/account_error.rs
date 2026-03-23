use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{AccountId, AccountStateError};

/// Describes why an `Account` aggregate operation failed.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<AccountId>),

    #[error(transparent)]
    State(#[from] AccountStateError),

    #[error("account is already opened")]
    AlreadyOpened,

    #[error("account is frozen")]
    Frozen,

    #[error("account has insufficient balance")]
    InsufficientBalance,

    #[error("account has insufficient available balance")]
    InsufficientAvailableBalance,

    #[error("transfer amount must be greater than zero")]
    ZeroTransferAmount,

    #[error("transfer target account must differ from source account")]
    SameTransferAccount,
}
