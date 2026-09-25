use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarHandleError {
    #[error("currency registrar handle cannot be empty")]
    Empty,
    #[error("currency registrar handle is too long")]
    TooLong,
    #[error("currency registrar handle contains an invalid character")]
    InvalidCharacter,
}
