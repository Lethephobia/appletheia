use std::{fmt, fmt::Display};

use uuid::Uuid;

use crate::core::Id;

use super::AggregateVersionTagIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AggregateVersionTagId(Id);

impl AggregateVersionTagId {
    pub fn new() -> Self {
        Self(Id::new())
    }

    pub fn value(self) -> Id {
        self.0
    }
}

impl Default for AggregateVersionTagId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Uuid> for AggregateVersionTagId {
    type Error = AggregateVersionTagIdError;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(Id::try_from(value)?))
    }
}

impl From<AggregateVersionTagId> for Uuid {
    fn from(value: AggregateVersionTagId) -> Self {
        value.0.value()
    }
}

impl From<Id> for AggregateVersionTagId {
    fn from(value: Id) -> Self {
        Self(value)
    }
}

impl Display for AggregateVersionTagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::IdError;
    use uuid::{Uuid, Version};

    #[test]
    fn new_generates_uuid_v7() {
        let uuid: Uuid = AggregateVersionTagId::new().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn default_generates_uuid_v7() {
        let uuid: Uuid = AggregateVersionTagId::default().into();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn try_from_accepts_uuid_v7() {
        let uuid = Uuid::now_v7();
        let aggregate_version_tag_id =
            AggregateVersionTagId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(Uuid::from(aggregate_version_tag_id), uuid);
    }

    #[test]
    fn try_from_rejects_non_uuid_v7() {
        let uuid = Uuid::nil();

        match AggregateVersionTagId::try_from(uuid) {
            Err(AggregateVersionTagIdError::Id(IdError::NotUuidV7(returned))) => {
                assert_eq!(returned, uuid)
            }
            other => {
                panic!("expected NotUuidV7 error via AggregateVersionTagIdError::Id, got {other:?}")
            }
        }
    }

    #[test]
    fn from_id_preserves_value() {
        let id = Id::new();
        let aggregate_version_tag_id = AggregateVersionTagId::from(id);

        assert_eq!(aggregate_version_tag_id.value(), id);
    }

    #[test]
    fn display_formats_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let aggregate_version_tag_id =
            AggregateVersionTagId::try_from(uuid).expect("uuidv7 should be accepted");

        assert_eq!(aggregate_version_tag_id.to_string(), uuid.to_string());
    }
}
