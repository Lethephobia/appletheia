use std::error::Error;

use thiserror::Error;

/// Error returned while writing public user profile read models.
#[derive(Debug, Error)]
pub enum UserPublicProfileWriterError {
    #[error("user public profile writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
