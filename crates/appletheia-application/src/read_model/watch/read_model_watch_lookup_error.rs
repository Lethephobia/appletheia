use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchLookupError {
    #[error("read model watch lookup backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
