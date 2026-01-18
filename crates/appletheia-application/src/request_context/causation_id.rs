use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::request_context::MessageId;
use appletheia_domain::event::EventId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CausationId(Uuid);

impl CausationId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<MessageId> for CausationId {
    fn from(value: MessageId) -> Self {
        Self(value.value())
    }
}

impl From<EventId> for CausationId {
    fn from(value: EventId) -> Self {
        Self(value.value())
    }
}

impl From<CausationId> for Uuid {
    fn from(value: CausationId) -> Self {
        value.0
    }
}

impl Display for CausationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
