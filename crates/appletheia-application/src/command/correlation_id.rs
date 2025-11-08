use std::{fmt, fmt::Display};

use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CorrelationId(pub Uuid);

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
