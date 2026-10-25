use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelWatchIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWatchId(Uuid);

impl ReadModelWatchId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelWatchId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelWatchId {
    type Error = ReadModelWatchIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelWatchIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelWatchId> for Uuid {
    fn from(value: ReadModelWatchId) -> Self {
        value.value()
    }
}

impl Display for ReadModelWatchId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
