use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{AccountBalanceError, AccountId};

/// Describes why an `Account` aggregate operation failed.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<AccountId>),

    #[error("account is already opened")]
    AlreadyOpened,

    #[error("account is frozen")]
    Frozen,

    #[error("account is closed")]
    Closed,

    #[error("account has insufficient balance")]
    InsufficientBalance,

    #[error("account has insufficient available balance")]
    InsufficientAvailableBalance,

    #[error("account has insufficient reserved balance")]
    InsufficientReservedBalance,

    #[error("account balance overflowed")]
    BalanceOverflow,

    #[error("account reserved balance exceeds total balance")]
    InvalidReservedBalance,

    #[error("account balance must be zero before closing")]
    BalanceRemaining,

    #[error("account reserved balance must be zero before closing")]
    ReservedBalanceRemaining,
}

impl From<AccountBalanceError> for AccountError {
    fn from(error: AccountBalanceError) -> Self {
        match error {
            AccountBalanceError::BalanceOverflow => Self::BalanceOverflow,
            AccountBalanceError::InsufficientBalance => Self::InsufficientBalance,
        }
    }
}
