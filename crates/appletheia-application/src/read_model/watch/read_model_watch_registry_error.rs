use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchRegistryError {
    #[error("read model watch registry backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
