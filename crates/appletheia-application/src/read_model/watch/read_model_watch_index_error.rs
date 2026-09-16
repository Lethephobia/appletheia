use std::error::Error;

use thiserror::Error;

use super::ReadModelWatchIndexExpiresAtError;

#[derive(Debug, Error)]
pub enum ReadModelWatchIndexError {
    #[error("failed to replace watch index registration: {0}")]
    Replace(#[source] Box<dyn Error + Send + Sync>),

    #[error("failed to find watch endpoints: {0}")]
    Find(#[source] Box<dyn Error + Send + Sync>),

    #[error(transparent)]
    ExpiresAt(#[from] ReadModelWatchIndexExpiresAtError),
}
