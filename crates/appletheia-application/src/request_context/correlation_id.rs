use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CorrelationId(Uuid);

impl CorrelationId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for CorrelationId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<CorrelationId> for Uuid {
    fn from(value: CorrelationId) -> Self {
        value.0
    }
}

impl Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
