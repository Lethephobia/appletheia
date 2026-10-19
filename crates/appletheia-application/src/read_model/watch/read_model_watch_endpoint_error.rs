use std::error::Error as StdError;

use thiserror::Error;

use super::{
    ReadModelWatchEndpointRegistryError, ReadModelWatchLookupError, ReadModelWatchRegistryError,
};

#[derive(Debug, Error)]
pub enum ReadModelWatchEndpointError {
    #[error(transparent)]
    EndpointRegistry(#[from] ReadModelWatchEndpointRegistryError),

    #[error(transparent)]
    Registry(#[from] ReadModelWatchRegistryError),

    #[error(transparent)]
    Lookup(#[from] ReadModelWatchLookupError),

    #[error("read model watch endpoint backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
