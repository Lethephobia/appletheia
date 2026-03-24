use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::UserDisplayNameError;

/// Represents an optional user-facing display name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct UserDisplayName(String);

impl UserDisplayName {
    /// Creates a new display name.
    pub fn new(value: String) -> Result<Self, UserDisplayNameError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(UserDisplayNameError::Empty);
        }

        if trimmed.chars().count() > 50 {
            return Err(UserDisplayNameError::TooLong);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the display-name value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for UserDisplayName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for UserDisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for UserDisplayName {
    type Err = UserDisplayNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for UserDisplayName {
    type Error = UserDisplayNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for UserDisplayName {
    type Error = UserDisplayNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<UserDisplayName> for String {
    fn from(value: UserDisplayName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{UserDisplayName, UserDisplayNameError};

    #[test]
    fn accepts_valid_display_name() {
        let display_name =
            UserDisplayName::try_from("  Alice Example  ").expect("display name should be valid");

        assert_eq!(display_name.value(), "Alice Example");
    }

    #[test]
    fn rejects_empty_display_name() {
        let error = UserDisplayName::try_from("   ").expect_err("empty display name should fail");

        assert!(matches!(error, UserDisplayNameError::Empty));
    }

    #[test]
    fn rejects_too_long_display_name() {
        let value = "a".repeat(51);
        let error = UserDisplayName::try_from(value).expect_err("display name should be too long");

        assert!(matches!(error, UserDisplayNameError::TooLong));
    }
}
