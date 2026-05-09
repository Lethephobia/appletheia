use thiserror::Error;

use crate::read_model::UserPrivateInfoReaderError;

/// Error returned while handling user-private information queries.
#[derive(Debug, Error)]
pub enum UserPrivateInfoQueryHandlerError {
    #[error("user private info store failed")]
    Store(#[from] UserPrivateInfoReaderError),
}
