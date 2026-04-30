use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::SagaInstanceIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SagaInstanceId(Uuid);

impl SagaInstanceId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for SagaInstanceId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for SagaInstanceId {
    type Error = SagaInstanceIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(SagaInstanceIdError::NotUuidV7(value)),
        }
    }
}

impl From<SagaInstanceId> for Uuid {
    fn from(value: SagaInstanceId) -> Self {
        value.value()
    }
}

impl Display for SagaInstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = SagaInstanceId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = SagaInstanceId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let saga_instance_id = SagaInstanceId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(saga_instance_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match SagaInstanceId::try_from(uuid) {
            Err(SagaInstanceIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let saga_instance_id = SagaInstanceId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(saga_instance_id.to_string(), uuid.to_string());
    }
}
