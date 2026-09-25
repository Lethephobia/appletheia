use thiserror::Error;

use super::{
    ReadModelWatchId, ReadModelWatchIndexError, ReadModelWatchSessionError, ReadModelWatchSessionId,
};

#[derive(Debug, Error)]
pub enum ReadModelWatchRegistryError {
    #[error("watch session was not found: {0}")]
    SessionNotFound(ReadModelWatchSessionId),

    #[error("watch is already registered with another session: {0}")]
    WatchAlreadyRegistered(ReadModelWatchId),

    #[error(transparent)]
    Session(#[from] ReadModelWatchSessionError),

    #[error(transparent)]
    Index(#[from] ReadModelWatchIndexError),
}
