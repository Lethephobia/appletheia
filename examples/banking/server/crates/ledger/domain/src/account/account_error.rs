use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{AccountBalanceError, AccountId, AccountStateError};

/// Describes why an `Account` aggregate operation failed.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<AccountId>),

    #[error(transparent)]
    State(#[from] AccountStateError),

    #[error("account is already opened")]
    AlreadyOpened,

    #[error(transparent)]
    AccountBalance(#[from] AccountBalanceError),

    #[error("account is closed")]
    Closed,

    #[error("account is frozen")]
    Frozen,

    #[error("account has insufficient reserved balance")]
    InsufficientReservedBalance,

    #[error("account has insufficient available balance")]
    InsufficientAvailableBalance,

    #[error("account balance must be zero before closing")]
    BalanceRemaining,

    #[error("account reserved balance must be zero before closing")]
    ReservedBalanceRemaining,

    #[error("account has insufficient balance")]
    InsufficientBalance,
}
