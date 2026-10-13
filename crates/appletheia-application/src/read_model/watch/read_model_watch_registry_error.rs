use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchRegistryError {
    #[error("read model watch registry backend error: {0}")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}
