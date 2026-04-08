use thiserror::Error;

/// Describes why a `CurrencyAmount` operation failed.
#[derive(Debug, Error)]
pub enum CurrencyAmountError {
    #[error("currency amount overflowed")]
    BalanceOverflow,

    #[error("insufficient currency amount")]
    InsufficientBalance,
}
