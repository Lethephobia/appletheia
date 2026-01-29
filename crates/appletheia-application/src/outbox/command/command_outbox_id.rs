use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::CommandOutboxIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CommandOutboxId(Uuid);

impl CommandOutboxId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for CommandOutboxId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for CommandOutboxId {
    type Error = CommandOutboxIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(CommandOutboxIdError::NotUuidV7(value)),
        }
    }
}

impl From<CommandOutboxId> for Uuid {
    fn from(value: CommandOutboxId) -> Self {
        value.value()
    }
}

impl Display for CommandOutboxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
