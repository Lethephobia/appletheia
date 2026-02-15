use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::SubjectKindError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubjectKind(String);

impl SubjectKind {
    pub const MAX_LENGTH: usize = 50;

    pub fn new(value: String) -> Result<Self, SubjectKindError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), SubjectKindError> {
        if value.is_empty() {
            return Err(SubjectKindError::Empty);
        }
        let len = value.len();
        if len > Self::MAX_LENGTH {
            return Err(SubjectKindError::TooLong {
                len,
                max: Self::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(SubjectKindError::InvalidFormat {
                value: value.to_owned(),
            });
        }
        Ok(())
    }
}

impl FromStr for SubjectKind {
    type Err = SubjectKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_owned())
    }
}

impl TryFrom<String> for SubjectKind {
    type Error = SubjectKindError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SubjectKind {
    type Error = SubjectKindError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl Display for SubjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        let err = SubjectKind::try_from("").expect_err("empty should be rejected");
        assert!(matches!(err, SubjectKindError::Empty));
    }

    #[test]
    fn rejects_invalid_format() {
        let err = SubjectKind::try_from("User").expect_err("invalid should be rejected");
        assert!(matches!(err, SubjectKindError::InvalidFormat { .. }));
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(SubjectKind::MAX_LENGTH + 1);
        let err = SubjectKind::try_from(long).expect_err("too long should be rejected");
        assert!(matches!(err, SubjectKindError::TooLong { .. }));
    }

    #[test]
    fn accepts_snake_case() {
        let kind = SubjectKind::try_from("api_key").expect("valid");
        assert_eq!(kind.value(), "api_key");
    }
}
