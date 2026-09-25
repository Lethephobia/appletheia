use thiserror::Error;

use crate::core::CurrencyAmountError;

/// Describes why an account balance transition failed.
#[derive(Debug, Error)]
pub enum AccountBalanceError {
    #[error(transparent)]
    Amount(#[from] CurrencyAmountError),

    #[error("account has insufficient reserved balance")]
    InsufficientReservedBalance,

    #[error("account reserved balance exceeds total balance")]
    InvalidReservedBalance,
}
