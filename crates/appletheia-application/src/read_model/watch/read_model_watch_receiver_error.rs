use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchReceiverError {
    #[error("read model watch receiver backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
