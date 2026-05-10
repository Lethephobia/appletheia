use thiserror::Error;

use crate::read_model::PublicAccountListReaderError;

/// Error returned by public account list query handlers.
#[derive(Debug, Error)]
pub enum PublicAccountListQueryHandlerError {
    #[error("public account list reader error")]
    Reader(#[from] PublicAccountListReaderError),
}
