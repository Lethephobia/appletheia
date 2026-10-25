use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::ReadModelChangeOutboxIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ReadModelChangeOutboxId(Uuid);

impl ReadModelChangeOutboxId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelChangeOutboxId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelChangeOutboxId {
    type Error = ReadModelChangeOutboxIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelChangeOutboxIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelChangeOutboxId> for Uuid {
    fn from(value: ReadModelChangeOutboxId) -> Self {
        value.value()
    }
}

impl Display for ReadModelChangeOutboxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
