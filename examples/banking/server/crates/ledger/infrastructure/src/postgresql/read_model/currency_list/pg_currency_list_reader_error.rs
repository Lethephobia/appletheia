use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PgCurrencyListReaderError {
    #[error("unknown currency status: {0}")]
    Status(String),
}
