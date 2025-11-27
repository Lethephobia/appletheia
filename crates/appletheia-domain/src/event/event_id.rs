use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::EventIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for EventId {
    type Error = EventIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(EventIdError::NotUuidV7(value)),
        }
    }
}

impl From<EventId> for Uuid {
    fn from(value: EventId) -> Self {
        value.value()
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::{Uuid, Version};

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = EventId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = EventId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(event_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match EventId::try_from(uuid) {
            Err(EventIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(event_id.to_string(), uuid.to_string());
    }
}
