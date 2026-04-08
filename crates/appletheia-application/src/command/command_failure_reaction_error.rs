use thiserror::Error;

use crate::command::CommandOwnedError;

/// Represents errors returned while preparing a command failure reaction.
#[derive(Debug, Error)]
pub enum CommandFailureReactionError {
    #[error("failed to prepare an owned command")]
    CommandOwned(#[from] CommandOwnedError),
}
