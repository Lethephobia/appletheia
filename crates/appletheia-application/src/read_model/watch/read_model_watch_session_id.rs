use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies one server-created active client watch session.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWatchSessionId(Uuid);

impl ReadModelWatchSessionId {
    /// Creates a new server-generated session identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ReadModelWatchSessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ReadModelWatchSessionId> for Uuid {
    fn from(value: ReadModelWatchSessionId) -> Self {
        value.value()
    }
}

impl Display for ReadModelWatchSessionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Version;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = ReadModelWatchSessionId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = ReadModelWatchSessionId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let session_id = ReadModelWatchSessionId(uuid);

        assert_eq!(session_id.to_string(), uuid.to_string());
        assert_eq!(Uuid::from(session_id), uuid);
    }
}
