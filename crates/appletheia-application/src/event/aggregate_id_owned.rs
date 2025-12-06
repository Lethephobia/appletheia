use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AggregateIdOwnedError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AggregateIdOwned(Uuid);

impl AggregateIdOwned {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl FromStr for AggregateIdOwned {
    type Err = AggregateIdOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::try_parse(s)
            .map(Self)
            .map_err(|source| AggregateIdOwnedError::InvalidUuid {
                value: s.to_owned(),
                source,
            })
    }
}

impl TryFrom<&str> for AggregateIdOwned {
    type Error = AggregateIdOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for AggregateIdOwned {
    type Error = AggregateIdOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Uuid::try_parse(&value) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(source) => Err(AggregateIdOwnedError::InvalidUuid { value, source }),
        }
    }
}

impl From<Uuid> for AggregateIdOwned {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl Display for AggregateIdOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_uuid() {
        let uuid = Uuid::nil();
        let owned = AggregateIdOwned::from(uuid);
        assert_eq!(owned.value(), uuid);
    }

    #[test]
    fn parses_from_str() {
        let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
        let owned: AggregateIdOwned = uuid_str.parse().expect("valid uuid string");
        assert_eq!(owned.to_string(), uuid_str);
    }

    #[test]
    fn rejects_invalid_uuid() {
        let err = AggregateIdOwned::try_from("not-a-uuid").expect_err("should fail");
        match err {
            AggregateIdOwnedError::InvalidUuid { value, .. } => {
                assert_eq!(value, "not-a-uuid");
            }
        }
    }
}
