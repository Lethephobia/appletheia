use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::UsernameError;

/// Represents a user's display handle.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Username(String);

impl Username {
    /// Creates a new username.
    pub fn new(value: String) -> Result<Self, UsernameError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(UsernameError::Empty);
        }

        if trimmed.chars().count() > 30 {
            return Err(UsernameError::TooLong);
        }

        if !trimmed.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        }) {
            return Err(UsernameError::InvalidCharacter);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the username value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for Username {
    type Err = UsernameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for Username {
    type Error = UsernameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for Username {
    type Error = UsernameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Username, UsernameError};

    #[test]
    fn accepts_valid_username() {
        let username = Username::try_from("  alice_example  ").expect("username should be valid");

        assert_eq!(username.value(), "alice_example");
    }

    #[test]
    fn rejects_empty_username() {
        let error = Username::try_from("   ").expect_err("empty username should fail");

        assert!(matches!(error, UsernameError::Empty));
    }

    #[test]
    fn rejects_too_long_username() {
        let value = "a".repeat(31);
        let error = Username::try_from(value).expect_err("username should be too long");

        assert!(matches!(error, UsernameError::TooLong));
    }

    #[test]
    fn rejects_invalid_character_username() {
        let error = Username::try_from("Alice Example").expect_err("username should be invalid");

        assert!(matches!(error, UsernameError::InvalidCharacter));
    }
}
