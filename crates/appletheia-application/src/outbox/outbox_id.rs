use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::OutboxIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxId(Uuid);

impl OutboxId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(self) -> Uuid {
        self.0
    }
}

impl Default for OutboxId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for OutboxId {
    type Error = OutboxIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(OutboxIdError::NotUuidV7(value)),
        }
    }
}

impl From<OutboxId> for Uuid {
    fn from(value: OutboxId) -> Self {
        value.value()
    }
}

impl Display for OutboxId {
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
        let uuid = OutboxId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = OutboxId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let outbox_id = OutboxId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(outbox_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match OutboxId::try_from(uuid) {
            Err(OutboxIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let outbox_id = OutboxId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(outbox_id.to_string(), uuid.to_string());
    }
}
