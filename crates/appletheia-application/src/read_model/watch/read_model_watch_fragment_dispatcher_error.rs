use thiserror::Error;

use super::{ReadModelWatchDispatchError, ReadModelWatchSessionId};

/// Reports a session delivery that prevents acknowledging a fragment change.
#[derive(Debug, Error)]
#[error("failed to dispatch read model fragment change to session {session_id}")]
pub struct ReadModelWatchFragmentDispatcherError {
    pub session_id: ReadModelWatchSessionId,
    #[source]
    pub source: ReadModelWatchDispatchError,
}
