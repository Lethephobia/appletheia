use thiserror::Error;

/// Describes why smallest-unit currency arithmetic failed.
#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum CurrencyAmountError {
    #[error("currency amount overflowed")]
    Overflow,

    #[error("currency amount is insufficient")]
    InsufficientAmount,
}
