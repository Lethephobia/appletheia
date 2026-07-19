use thiserror::Error;

use crate::read_model::PublicUserListReaderError;

/// Error returned while handling public user list queries.
#[derive(Debug, Error)]
pub enum PublicUserListQueryHandlerError {
    #[error("public user list reader failed")]
    Reader(#[from] PublicUserListReaderError),
}
