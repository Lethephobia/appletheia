use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarDescriptionError {
    #[error("currency registrar description cannot be empty")]
    Empty,
    #[error("currency registrar description is too long")]
    TooLong,
}
