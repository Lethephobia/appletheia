use serde::{Deserialize, Serialize};

/// Contains application-safe information about a failed subscription refresh.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelWatchFailure {
    pub code: String,
    pub retryable: bool,
}
