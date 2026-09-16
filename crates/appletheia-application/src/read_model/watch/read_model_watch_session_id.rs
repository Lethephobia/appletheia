use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies one local client session.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWatchSessionId(Uuid);

impl ReadModelWatchSessionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelWatchSessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ReadModelWatchSessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
