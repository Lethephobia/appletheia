use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::request_context::MessageId;
use appletheia_domain::event::EventId;

/// Identifies the message or event that directly caused a downstream action.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CausationId(Uuid);

impl CausationId {
    /// Returns the raw UUID value.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_message_id_uses_same_uuid() {
        let message_id = MessageId::from(Uuid::now_v7());
        let causation_id = CausationId::from(message_id);

        assert_eq!(causation_id.value(), message_id.value());
    }

    #[test]
    fn from_event_id_uses_same_uuid() {
        let event_id = EventId::new();
        let causation_id = CausationId::from(event_id);

        assert_eq!(causation_id.value(), event_id.value());
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let message_id = MessageId::from(Uuid::now_v7());
        let causation_id = CausationId::from(message_id);

        assert_eq!(causation_id.to_string(), causation_id.value().to_string());
    }
}
