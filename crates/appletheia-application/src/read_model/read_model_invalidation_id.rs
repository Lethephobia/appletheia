use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelInvalidationIdError;

/// Identifies one durable read-model invalidation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "Uuid", into = "Uuid")]
pub struct ReadModelInvalidationId(Uuid);

impl ReadModelInvalidationId {
    /// Creates a new invalidation identifier.
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
        Display::fmt(&self.0, formatter)
    }
}
