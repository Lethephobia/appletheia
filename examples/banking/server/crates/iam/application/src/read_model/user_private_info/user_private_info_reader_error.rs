use std::error::Error;

use thiserror::Error;

/// Error returned while loading user-private read models.
#[derive(Debug, Error)]
pub enum UserPrivateInfoReaderError {
    #[error("user private info reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
