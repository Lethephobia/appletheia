use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies one watch.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWatchId(Uuid);

impl ReadModelWatchId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelWatchId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ReadModelWatchId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
