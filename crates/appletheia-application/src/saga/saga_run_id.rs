use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::SagaRunIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SagaRunId(Uuid);

impl SagaRunId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for SagaRunId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for SagaRunId {
    type Error = SagaRunIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(SagaRunIdError::NotUuidV7(value)),
        }
    }
}

impl From<SagaRunId> for Uuid {
    fn from(value: SagaRunId) -> Self {
        value.value()
    }
}

impl Display for SagaRunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = SagaRunId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = SagaRunId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let saga_run_id = SagaRunId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(saga_run_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match SagaRunId::try_from(uuid) {
            Err(SagaRunIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let saga_run_id = SagaRunId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(saga_run_id.to_string(), uuid.to_string());
    }
}
