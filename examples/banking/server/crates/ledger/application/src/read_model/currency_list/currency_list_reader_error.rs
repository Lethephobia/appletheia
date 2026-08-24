use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyListReaderError {
    #[error("currency list persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
