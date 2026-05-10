use crate::read_model::CurrencyListReaderError;
use thiserror::Error;

/// Error returned while handling currency list queries.
#[derive(Debug, Error)]
pub enum CurrencyListQueryHandlerError {
    #[error("currency list reader failed")]
    Reader(#[from] CurrencyListReaderError),
}
