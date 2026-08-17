use crate::read_model::watch::ReadModelWatchSessionId;

/// Requests registration of a watchable query result for one active client session.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ReadModelWatchOptions {
    pub session_id: ReadModelWatchSessionId,
}
