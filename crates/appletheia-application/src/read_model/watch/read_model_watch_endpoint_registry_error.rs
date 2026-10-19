use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchEndpointRegistryError {
    #[error("read model watch endpoint registry backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
