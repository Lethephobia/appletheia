use appletheia::domain::AggregateError;
use thiserror::Error;

use crate::core::CurrencyAmountError;

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

    #[error("account has insufficient balance")]
    InsufficientBalance,

    #[error("account has insufficient reserved balance")]
    InsufficientReservedBalance,

    #[error("account balance overflowed")]
    BalanceOverflow,

    #[error("account reserved balance exceeds total balance")]
    InvalidReservedBalance,
}

impl From<CurrencyAmountError> for AccountError {
    fn from(error: CurrencyAmountError) -> Self {
        match error {
            CurrencyAmountError::BalanceOverflow => Self::BalanceOverflow,
            CurrencyAmountError::InsufficientBalance => Self::InsufficientBalance,
        }
    }
}
