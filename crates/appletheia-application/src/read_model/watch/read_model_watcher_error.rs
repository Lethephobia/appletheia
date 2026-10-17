use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatcherError {
    #[error("read model watcher backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
