use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyFragmentWriterError {
    #[error("currency fragment persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
