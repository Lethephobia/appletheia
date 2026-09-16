use thiserror::Error;

use super::ReadModelWatchId;

#[derive(Debug, Error)]
pub enum ReadModelWatchSessionError {
    #[error("watch session is closing")]
    Closing,

    #[error("watch is being removed: {0}")]
    WatchRemoving(ReadModelWatchId),
}
