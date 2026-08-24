use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PgCurrencyFragmentWriterError {
    #[error("unknown currency status: {0}")]
    Status(String),
}
