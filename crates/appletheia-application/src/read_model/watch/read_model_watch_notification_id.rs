use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelWatchNotificationIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWatchNotificationId(Uuid);

impl ReadModelWatchNotificationId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelWatchNotificationId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelWatchNotificationId {
    type Error = ReadModelWatchNotificationIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelWatchNotificationIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelWatchNotificationId> for Uuid {
    fn from(value: ReadModelWatchNotificationId) -> Self {
        value.value()
    }
}

impl Display for ReadModelWatchNotificationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
