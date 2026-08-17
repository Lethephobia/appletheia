use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

use super::ReadModelFragmentChangeIdError;

/// Identifies one durable source-fragment change envelope.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "Uuid", into = "Uuid")]
pub struct ReadModelFragmentChangeId(Uuid);

impl ReadModelFragmentChangeId {
    /// Creates a new fragment-change identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the raw UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelFragmentChangeId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ReadModelFragmentChangeId {
    type Error = ReadModelFragmentChangeIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ReadModelFragmentChangeIdError::NotUuidV7(value)),
        }
    }
}

impl From<ReadModelFragmentChangeId> for Uuid {
    fn from(value: ReadModelFragmentChangeId) -> Self {
        value.value()
    }
}

impl Display for ReadModelFragmentChangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rejects_a_non_v7_uuid() {
        let serialized = format!("\"{}\"", Uuid::nil());
        let result = serde_json::from_str::<ReadModelFragmentChangeId>(&serialized);

        assert!(result.is_err());
    }
}
