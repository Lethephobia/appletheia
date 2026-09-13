use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::ReadModelInvalidationOutboxIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ReadModelInvalidationOutboxId(Uuid);

impl ReadModelInvalidationOutboxId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelInvalidationOutboxId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelInvalidationOutboxId {
    type Error = ReadModelInvalidationOutboxIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelInvalidationOutboxIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelInvalidationOutboxId> for Uuid {
    fn from(value: ReadModelInvalidationOutboxId) -> Self {
        value.value()
    }
}

impl Display for ReadModelInvalidationOutboxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
