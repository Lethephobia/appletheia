use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::OutboxRelayIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OutboxRelayId(Uuid);

impl OutboxRelayId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for OutboxRelayId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for OutboxRelayId {
    type Error = OutboxRelayIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(OutboxRelayIdError::NotUuidV7(value)),
        }
    }
}

impl From<OutboxRelayId> for Uuid {
    fn from(value: OutboxRelayId) -> Self {
        value.value()
    }
}

impl Display for OutboxRelayId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
