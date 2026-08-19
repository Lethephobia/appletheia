use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelWatchSubscriptionIdError;

/// Identifies one logical query subscription within a watch session.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "Uuid", into = "Uuid")]
pub struct ReadModelWatchSubscriptionId(Uuid);

impl ReadModelWatchSubscriptionId {
    /// Creates a new subscription identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the raw UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelWatchSubscriptionId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelWatchSubscriptionId {
    type Error = ReadModelWatchSubscriptionIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelWatchSubscriptionIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelWatchSubscriptionId> for Uuid {
    fn from(value: ReadModelWatchSubscriptionId) -> Self {
        value.value()
    }
}

impl Display for ReadModelWatchSubscriptionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}
