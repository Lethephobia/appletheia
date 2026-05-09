use std::error::Error;

use thiserror::Error;

/// Error returned while loading public user profile read models.
#[derive(Debug, Error)]
pub enum UserPublicProfileReaderError {
    #[error("user public profile reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
