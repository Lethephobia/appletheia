use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::ProjectorProcessedEventIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ProjectorProcessedEventId(Uuid);

impl ProjectorProcessedEventId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ProjectorProcessedEventId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for ProjectorProcessedEventId {
    type Error = ProjectorProcessedEventIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(ProjectorProcessedEventIdError::NotUuidV7(value)),
        }
    }
}

impl From<ProjectorProcessedEventId> for Uuid {
    fn from(value: ProjectorProcessedEventId) -> Self {
        value.value()
    }
}

impl Display for ProjectorProcessedEventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = ProjectorProcessedEventId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = ProjectorProcessedEventId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let id = ProjectorProcessedEventId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match ProjectorProcessedEventId::try_from(uuid) {
            Err(ProjectorProcessedEventIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let id = ProjectorProcessedEventId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(id.to_string(), uuid.to_string());
    }
}
