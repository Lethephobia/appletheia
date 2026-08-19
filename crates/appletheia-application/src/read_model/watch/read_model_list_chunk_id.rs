use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelListChunkIdError;

/// Identifies one reloadable list chunk.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "Uuid", into = "Uuid")]
pub struct ReadModelListChunkId(Uuid);

impl ReadModelListChunkId {
    /// Creates a new chunk identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the raw UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelListChunkId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelListChunkId {
    type Error = ReadModelListChunkIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelListChunkIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelListChunkId> for Uuid {
    fn from(value: ReadModelListChunkId) -> Self {
        value.value()
    }
}

impl Display for ReadModelListChunkId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}
