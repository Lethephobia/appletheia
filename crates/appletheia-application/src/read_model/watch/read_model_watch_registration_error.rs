use thiserror::Error;

use crate::read_model::ReadModelNameOwned;

use super::ReadModelWatchSessionId;

/// Reports a failure to install a snapshot selection into an active session.
#[derive(Debug, Error)]
pub enum ReadModelWatchRegistrationError {
    #[error("read model watch registration is unavailable")]
    Unavailable,

    #[error("read model watch session is not active: {0}")]
    SessionNotFound(ReadModelWatchSessionId),

    #[error("watch session read model mismatch: expected {expected}, got {actual}")]
    ReadModelMismatch {
        expected: ReadModelNameOwned,
        actual: ReadModelNameOwned,
    },
}
