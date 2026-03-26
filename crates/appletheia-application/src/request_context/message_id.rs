use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies a single inbound message or command execution attempt.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(Uuid);

impl MessageId {
    /// Creates a new message ID backed by a freshly generated UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the raw UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for MessageId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<MessageId> for Uuid {
    fn from(value: MessageId) -> Self {
        value.0
    }
}

impl Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Version;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = MessageId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = MessageId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn from_uuid_round_trips() {
        let uuid = Uuid::now_v7();
        let message_id = MessageId::from(uuid);

        assert_eq!(Uuid::from(message_id), uuid);
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let message_id = MessageId::from(uuid);

        assert_eq!(message_id.to_string(), uuid.to_string());
    }
}
