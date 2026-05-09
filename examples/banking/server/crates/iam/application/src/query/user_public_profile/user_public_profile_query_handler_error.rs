use thiserror::Error;

use crate::read_model::UserPublicProfileReaderError;

/// Error returned while handling public user profile queries.
#[derive(Debug, Error)]
pub enum UserPublicProfileQueryHandlerError {
    #[error("user public profile reader failed")]
    Reader(#[from] UserPublicProfileReaderError),
}
