use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::UsernameError;

/// Represents a validated username.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Username(String);

impl Username {
    /// Creates a username from user input.
    pub fn new(value: String) -> Result<Self, UsernameError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(UsernameError::Empty);
        }

        if normalized.len() > 100 {
            return Err(UsernameError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated username.
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

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for Username {
    type Error = UsernameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Username {
    type Error = UsernameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Username, UsernameError};

    #[test]
    fn accepts_valid_username() {
        let username = Username::try_from("  Alice Example  ").expect("username should be valid");

        assert_eq!(username.value(), "Alice Example");
    }

    #[test]
    fn rejects_empty_username() {
        let error = Username::try_from("   ").expect_err("empty username should fail");

        assert!(matches!(error, UsernameError::Empty));
    }

    #[test]
    fn rejects_too_long_username() {
        let value = "a".repeat(101);
        let error = Username::try_from(value).expect_err("username should be too long");

        assert!(matches!(error, UsernameError::TooLong));
    }
}
