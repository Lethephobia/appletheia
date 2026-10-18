use std::error::Error as StdError;

use thiserror::Error;

use super::ReadModelWatchRegistryError;

#[derive(Debug, Error)]
pub enum ReadModelWatchEndpointError {
    #[error(transparent)]
    Registry(#[from] ReadModelWatchRegistryError),

    #[error("read model watch endpoint backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
