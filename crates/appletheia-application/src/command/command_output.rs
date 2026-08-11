use serde::{Serialize, de::DeserializeOwned};

use super::CommandReplayOutput;

/// Defines the replay-safe representation of a successfully handled command.
pub trait CommandOutput: Send + 'static {
    /// Output returned when a stored idempotency result is replayed.
    type ReplayOutput: Serialize + DeserializeOwned + Send + 'static;

    /// Returns the value that is safe to persist for idempotent replay.
    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput>;
}
