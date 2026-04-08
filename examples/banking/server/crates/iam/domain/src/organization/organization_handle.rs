use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::OrganizationHandleError;

/// Represents a validated organization handle.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OrganizationHandle(String);

impl OrganizationHandle {
    /// Creates a new organization handle.
    pub fn new(value: String) -> Result<Self, OrganizationHandleError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrganizationHandleError::Empty);
        }

        if trimmed.chars().count() > 64 {
            return Err(OrganizationHandleError::TooLong);
        }

        if !trimmed.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '_'
                || character == '-'
        }) {
            return Err(OrganizationHandleError::InvalidCharacter);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the validated handle.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OrganizationHandle {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OrganizationHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OrganizationHandle {
    type Err = OrganizationHandleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for OrganizationHandle {
    type Error = OrganizationHandleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for OrganizationHandle {
    type Error = OrganizationHandleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OrganizationHandle> for String {
    fn from(value: OrganizationHandle) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationHandle, OrganizationHandleError};

    #[test]
    fn accepts_valid_handle() {
        let handle =
            OrganizationHandle::try_from("  acme-labs_2026  ").expect("handle should be valid");

        assert_eq!(handle.value(), "acme-labs_2026");
    }

    #[test]
    fn rejects_empty_handle() {
        let error = OrganizationHandle::try_from("   ").expect_err("empty handle should fail");

        assert!(matches!(error, OrganizationHandleError::Empty));
    }

    #[test]
    fn rejects_too_long_handle() {
        let value = "a".repeat(65);
        let error = OrganizationHandle::try_from(value).expect_err("handle should be too long");

        assert!(matches!(error, OrganizationHandleError::TooLong));
    }

    #[test]
    fn rejects_invalid_character_handle() {
        let error = OrganizationHandle::try_from("Acme Labs").expect_err("handle should fail");

        assert!(matches!(error, OrganizationHandleError::InvalidCharacter));
    }
}
