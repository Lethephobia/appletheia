use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SubjectIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubjectId(Uuid);

impl SubjectId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for SubjectId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for SubjectId {
    type Err = SubjectIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::try_parse(s)
            .map(Self)
            .map_err(|source| SubjectIdError::InvalidUuid {
                value: s.to_owned(),
                source,
            })
    }
}

impl TryFrom<&str> for SubjectId {
    type Error = SubjectIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for SubjectId {
    type Error = SubjectIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::try_parse(&value)
            .map(Self)
            .map_err(|source| SubjectIdError::InvalidUuid { value, source })
    }
}

impl Display for SubjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_uuid() {
        let err = SubjectId::try_from("not-a-uuid").expect_err("invalid should be rejected");
        assert!(matches!(err, SubjectIdError::InvalidUuid { .. }));
    }

    #[test]
    fn accepts_uuid() {
        let uuid = Uuid::nil();
        let id = SubjectId::from(uuid);
        assert_eq!(id.value(), uuid);
    }
}
