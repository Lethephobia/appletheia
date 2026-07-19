use std::error::Error;

use thiserror::Error;

/// Error returned while loading public user list read models.
#[derive(Debug, Error)]
pub enum PublicUserListReaderError {
    #[error("public user list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
