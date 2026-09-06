use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::CommandFailureOutboxIdError;

/// Identifies one terminal command-failure outbox entry.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CommandFailureOutboxId(Uuid);

impl CommandFailureOutboxId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for CommandFailureOutboxId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for CommandFailureOutboxId {
    type Error = CommandFailureOutboxIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(CommandFailureOutboxIdError::NotUuidV7(value)),
        }
    }
}

impl Display for CommandFailureOutboxId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
