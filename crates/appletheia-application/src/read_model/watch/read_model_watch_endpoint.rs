use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies one process lifetime as a delivery destination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWatchEndpoint(Uuid);

impl ReadModelWatchEndpoint {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelWatchEndpoint {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ReadModelWatchEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
