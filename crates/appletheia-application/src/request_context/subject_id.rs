use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::SubjectIdError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubjectId(String);

impl SubjectId {
    pub const MAX_LENGTH: usize = 200;

    pub fn new(value: String) -> Result<Self, SubjectIdError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), SubjectIdError> {
        if value.is_empty() {
            return Err(SubjectIdError::Empty);
        }
        let len = value.len();
        if len > Self::MAX_LENGTH {
            return Err(SubjectIdError::TooLong {
                len,
                max: Self::MAX_LENGTH,
            });
        }
        Ok(())
    }
}

impl TryFrom<String> for SubjectId {
    type Error = SubjectIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SubjectId {
    type Error = SubjectIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl Display for SubjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        let err = SubjectId::try_from("").expect_err("empty should be rejected");
        assert!(matches!(err, SubjectIdError::Empty));
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(SubjectId::MAX_LENGTH + 1);
        let err = SubjectId::try_from(long).expect_err("too long should be rejected");
        assert!(matches!(err, SubjectIdError::TooLong { .. }));
    }

    #[test]
    fn accepts_non_empty() {
        let id = SubjectId::try_from("user-123").expect("non-empty");
        assert_eq!(id.value(), "user-123");
    }
}
