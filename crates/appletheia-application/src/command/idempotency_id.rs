use std::{fmt, fmt::Display};

use uuid::{Uuid, Version};

use super::IdempotencyIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdempotencyId(Uuid);

impl IdempotencyId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for IdempotencyId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for IdempotencyId {
    type Error = IdempotencyIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.get_version() {
            Some(Version::SortRand) => Ok(Self(value)),
            _ => Err(IdempotencyIdError::NotUuidV7(value)),
        }
    }
}

impl From<IdempotencyId> for Uuid {
    fn from(value: IdempotencyId) -> Self {
        value.value()
    }
}

impl Display for IdempotencyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_uuid_v7() {
        let uuid = IdempotencyId::new().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid = IdempotencyId::default().value();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let id = IdempotencyId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match IdempotencyId::try_from(uuid) {
            Err(IdempotencyIdError::NotUuidV7(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV7 error, got {other:?}"),
        }
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let id = IdempotencyId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(id.to_string(), uuid.to_string());
    }
}
