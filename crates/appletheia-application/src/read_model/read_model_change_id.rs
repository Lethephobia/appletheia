use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelChangeIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelChangeId(Uuid);

impl ReadModelChangeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelChangeId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelChangeId {
    type Error = ReadModelChangeIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelChangeIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelChangeId> for Uuid {
    fn from(value: ReadModelChangeId) -> Self {
        value.value()
    }
}

impl Display for ReadModelChangeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
