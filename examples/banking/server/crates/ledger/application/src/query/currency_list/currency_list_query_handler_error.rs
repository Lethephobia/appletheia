use appletheia::application::Retryability;
use thiserror::Error;

use crate::read_model::CurrencyListReaderError;

#[derive(Debug, Error)]
pub enum CurrencyListQueryHandlerError {
    #[error(transparent)]
    Reader(#[from] CurrencyListReaderError),
}

impl Retryability for CurrencyListQueryHandlerError {
    fn is_retryable(&self) -> bool {
        true
    }
}
