use thiserror::Error;

/// Describes why a currency amount could not be converted into token base units.
#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum TokenAmountConversionError {
    #[error("token decimal scale overflowed")]
    DecimalScaleOverflow,

    #[error("token amount overflowed")]
    AmountOverflow,

    #[error("currency amount cannot be represented exactly in token base units")]
    InexactAmount,
}
