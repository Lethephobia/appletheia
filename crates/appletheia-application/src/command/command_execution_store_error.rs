use std::error::Error;

use thiserror::Error;

/// Reports invalid transitions or persistence failures in command execution tracking.
#[derive(Debug, Error)]
pub enum CommandExecutionStoreError {
    #[error("invalid command execution state transition")]
    InvalidStateTransition,

    #[error("command execution persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
