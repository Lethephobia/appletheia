use crate::read_model::CurrencyListItemReaderError;
use thiserror::Error;

/// Error returned while handling currency list queries.
#[derive(Debug, Error)]
pub enum CurrencyListQueryHandlerError {
    #[error("currency list store failed")]
    Store(#[from] CurrencyListItemReaderError),
}
