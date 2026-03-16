use std::borrow::Borrow;
use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{RelationName, RelationNameOwnedError};

/// Owns a validated relation name for runtime authorization data.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelationNameOwned(String);

impl RelationNameOwned {
    pub fn new(value: String) -> Result<Self, RelationNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), RelationNameOwnedError> {
        if value.is_empty() {
            return Err(RelationNameOwnedError::Empty);
        }

        let len = value.len();
        if len > RelationName::MAX_LENGTH {
            return Err(RelationNameOwnedError::TooLong {
                len,
                max: RelationName::MAX_LENGTH,
            });
        }

        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(RelationNameOwnedError::InvalidFormat {
                value: value.to_owned(),
            });
        }

        Ok(())
    }
}

impl FromStr for RelationNameOwned {
    type Err = RelationNameOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_owned())
    }
}

impl TryFrom<&str> for RelationNameOwned {
    type Error = RelationNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for RelationNameOwned {
    type Error = RelationNameOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<RelationName> for RelationNameOwned {
    fn from(value: RelationName) -> Self {
        Self(value.value().to_owned())
    }
}

impl Display for RelationNameOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl AsRef<str> for RelationNameOwned {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Borrow<str> for RelationNameOwned {
    fn borrow(&self) -> &str {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::RelationNameOwned;
    use crate::authorization::{RelationName, RelationNameOwnedError};

    #[test]
    fn accepts_snake_case() {
        let relation = RelationNameOwned::try_from("viewer").expect("valid");
        assert_eq!(relation.value(), "viewer");
    }

    #[test]
    fn rejects_empty() {
        let err = RelationNameOwned::try_from("").expect_err("empty should be rejected");
        assert!(matches!(err, RelationNameOwnedError::Empty));
    }

    #[test]
    fn rejects_invalid_format() {
        let err = RelationNameOwned::try_from("Viewer").expect_err("invalid should be rejected");
        assert!(matches!(err, RelationNameOwnedError::InvalidFormat { .. }));
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(RelationName::MAX_LENGTH + 1);
        let err = RelationNameOwned::try_from(long).expect_err("too long should be rejected");
        assert!(matches!(err, RelationNameOwnedError::TooLong { .. }));
    }

    #[test]
    fn from_relation_name_copies_value() {
        let relation = RelationNameOwned::from(RelationName::new("editor"));

        assert_eq!(relation.value(), "editor");
    }
}
