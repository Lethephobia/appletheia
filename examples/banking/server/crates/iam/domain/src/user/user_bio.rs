use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::UserBioError;

/// Represents a user-facing profile bio.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct UserBio(String);

impl UserBio {
    /// Creates a new bio value.
    pub fn new(value: String) -> Result<Self, UserBioError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(UserBioError::Empty);
        }

        if trimmed.chars().count() > 280 {
            return Err(UserBioError::TooLong);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the bio value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for UserBio {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for UserBio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for UserBio {
    type Err = UserBioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for UserBio {
    type Error = UserBioError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for UserBio {
    type Error = UserBioError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<UserBio> for String {
    fn from(value: UserBio) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{UserBio, UserBioError};

    #[test]
    fn accepts_valid_bio() {
        let bio = UserBio::try_from("  Banking enthusiast  ").expect("bio should be valid");

        assert_eq!(bio.value(), "Banking enthusiast");
    }

    #[test]
    fn rejects_empty_bio() {
        let error = UserBio::try_from("   ").expect_err("empty bio should fail");

        assert!(matches!(error, UserBioError::Empty));
    }

    #[test]
    fn rejects_too_long_bio() {
        let value = "a".repeat(281);
        let error = UserBio::try_from(value).expect_err("bio should be too long");

        assert!(matches!(error, UserBioError::TooLong));
    }
}
