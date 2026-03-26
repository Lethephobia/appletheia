use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::RoleNameError;

/// Represents a stable role name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct RoleName(String);

impl RoleName {
    /// Creates a new role name.
    pub fn new(value: String) -> Result<Self, RoleNameError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(RoleNameError::Empty);
        }

        if trimmed.chars().count() > 64 {
            return Err(RoleNameError::TooLong);
        }

        if !trimmed.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        }) {
            return Err(RoleNameError::InvalidCharacter);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the role-name value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RoleName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for RoleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for RoleName {
    type Err = RoleNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for RoleName {
    type Error = RoleNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for RoleName {
    type Error = RoleNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<RoleName> for String {
    fn from(value: RoleName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{RoleName, RoleNameError};

    #[test]
    fn accepts_valid_role_name() {
        let role_name = RoleName::try_from("  admin_user  ").expect("role name should be valid");

        assert_eq!(role_name.value(), "admin_user");
    }

    #[test]
    fn rejects_empty_role_name() {
        let error = RoleName::try_from("  ").expect_err("empty role name should fail");

        assert!(matches!(error, RoleNameError::Empty));
    }

    #[test]
    fn rejects_too_long_role_name() {
        let error = RoleName::try_from("a".repeat(65)).expect_err("role name should be too long");

        assert!(matches!(error, RoleNameError::TooLong));
    }

    #[test]
    fn rejects_invalid_character_role_name() {
        let error = RoleName::try_from("Admin").expect_err("role name should be invalid");

        assert!(matches!(error, RoleNameError::InvalidCharacter));
    }
}
