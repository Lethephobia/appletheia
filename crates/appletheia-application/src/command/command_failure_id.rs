use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::CommandFailureIdError;

/// Identifies one terminal command-failure notification.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandFailureId(Uuid);

impl CommandFailureId {
    /// Creates a new command failure ID backed by a freshly generated UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the raw UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for CommandFailureId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for CommandFailureId {
    type Error = CommandFailureIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(CommandFailureIdError::NotUuidV7(value)),
        }
    }
}

impl From<CommandFailureId> for Uuid {
    fn from(value: CommandFailureId) -> Self {
        value.value()
    }
}

impl Display for CommandFailureId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = CommandFailureId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = CommandFailureId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let failure_id = CommandFailureId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(failure_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match CommandFailureId::try_from(uuid) {
            Err(CommandFailureIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let failure_id = CommandFailureId::new();

        assert_eq!(failure_id.to_string(), failure_id.value().to_string());
    }
}
