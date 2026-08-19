use super::{ReadModelWatchCloseReason, ReadModelWatchFailure};

/// Selects whether a failed refresh keeps or closes its subscription.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadModelWatchRefreshError {
    Failed(ReadModelWatchFailure),
    Closed(ReadModelWatchCloseReason),
}
