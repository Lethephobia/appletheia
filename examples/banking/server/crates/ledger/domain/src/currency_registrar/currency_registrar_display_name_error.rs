use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarDisplayNameError {
    #[error("currency registrar display name cannot be empty")]
    Empty,
    #[error("currency registrar display name is too long")]
    TooLong,
}
