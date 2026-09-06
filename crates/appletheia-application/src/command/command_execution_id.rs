use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::CommandExecutionIdError;

/// Identifies one durable command execution record.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CommandExecutionId(Uuid);

impl CommandExecutionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for CommandExecutionId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for CommandExecutionId {
    type Error = CommandExecutionIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(CommandExecutionIdError::NotUuidV7(value)),
        }
    }
}

impl From<CommandExecutionId> for Uuid {
    fn from(value: CommandExecutionId) -> Self {
        value.value()
    }
}

impl Display for CommandExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = CommandExecutionId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = CommandExecutionId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let execution_id = CommandExecutionId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(execution_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match CommandExecutionId::try_from(uuid) {
            Err(CommandExecutionIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let execution_id = CommandExecutionId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(execution_id.to_string(), uuid.to_string());
    }
}
