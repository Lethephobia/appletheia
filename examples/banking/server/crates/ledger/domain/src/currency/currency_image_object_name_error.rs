/// Describes why a currency image object name cannot be validated.
#[derive(Debug, thiserror::Error)]
pub enum CurrencyImageObjectNameError {
    #[error("currency image object name is empty")]
    Empty,

    #[error("currency image object name format is invalid")]
    InvalidFormat,
}
