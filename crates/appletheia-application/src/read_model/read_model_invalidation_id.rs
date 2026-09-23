use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelInvalidationIdError;

/// Identifies one terminal read-model invalidation notification.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelInvalidationId(Uuid);

impl ReadModelInvalidationId {
    /// Creates a new read-model invalidation ID backed by a freshly generated UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the raw UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelInvalidationId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelInvalidationId {
    type Error = ReadModelInvalidationIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelInvalidationIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelInvalidationId> for Uuid {
    fn from(value: ReadModelInvalidationId) -> Self {
        value.value()
    }
}

impl Display for ReadModelInvalidationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = ReadModelInvalidationId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = ReadModelInvalidationId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let failure_id =
            ReadModelInvalidationId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(failure_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match ReadModelInvalidationId::try_from(uuid) {
            Err(ReadModelInvalidationIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let failure_id = ReadModelInvalidationId::new();

        assert_eq!(failure_id.to_string(), failure_id.value().to_string());
    }
}
