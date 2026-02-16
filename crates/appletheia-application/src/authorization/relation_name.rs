use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::RelationNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelationName(String);

impl RelationName {
    pub const MAX_LENGTH: usize = 50;

    pub fn new(value: String) -> Result<Self, RelationNameError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), RelationNameError> {
        if value.is_empty() {
            return Err(RelationNameError::Empty);
        }
        let len = value.len();
        if len > Self::MAX_LENGTH {
            return Err(RelationNameError::TooLong {
                len,
                max: Self::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(RelationNameError::InvalidFormat {
                value: value.to_owned(),
            });
        }
        Ok(())
    }
}

impl FromStr for RelationName {
    type Err = RelationNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_owned())
    }
}

impl TryFrom<&str> for RelationName {
    type Error = RelationNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for RelationName {
    type Error = RelationNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for RelationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        let err = RelationName::try_from("").expect_err("empty should be rejected");
        assert!(matches!(err, RelationNameError::Empty));
    }

    #[test]
    fn rejects_invalid_format() {
        let err = RelationName::try_from("Viewer").expect_err("invalid should be rejected");
        assert!(matches!(err, RelationNameError::InvalidFormat { .. }));
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(RelationName::MAX_LENGTH + 1);
        let err = RelationName::try_from(long).expect_err("too long should be rejected");
        assert!(matches!(err, RelationNameError::TooLong { .. }));
    }

    #[test]
    fn accepts_snake_case() {
        let relation = RelationName::try_from("viewer").expect("valid");
        assert_eq!(relation.value(), "viewer");
    }
}

