use appletheia::domain::AggregateError;
use thiserror::Error;

use crate::core::CurrencyAmountError;

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

    #[error("account owner is not assigned")]
    OwnerNotAssigned,
}

impl From<CurrencyAmountError> for AccountError {
    fn from(error: CurrencyAmountError) -> Self {
        match error {
            CurrencyAmountError::BalanceOverflow => Self::BalanceOverflow,
            CurrencyAmountError::InsufficientBalance => Self::InsufficientBalance,
        }
    }
}
