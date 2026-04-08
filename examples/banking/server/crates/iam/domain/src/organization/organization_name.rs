use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::OrganizationNameError;

/// Represents a validated organization name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationName(String);

impl OrganizationName {
    /// Creates an organization name from user input.
    pub fn new(value: String) -> Result<Self, OrganizationNameError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(OrganizationNameError::Empty);
        }

        if normalized.len() > 100 {
            return Err(OrganizationNameError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OrganizationName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OrganizationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OrganizationName {
    type Err = OrganizationNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for OrganizationName {
    type Error = OrganizationNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for OrganizationName {
    type Error = OrganizationNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationName, OrganizationNameError};

    #[test]
    fn accepts_valid_name() {
        let name = OrganizationName::try_from("  Acme Labs  ").expect("name should be valid");

        assert_eq!(name.value(), "Acme Labs");
    }

    #[test]
    fn rejects_empty_name() {
        let error = OrganizationName::try_from("   ").expect_err("empty name should fail");

        assert!(matches!(error, OrganizationNameError::Empty));
    }

    #[test]
    fn rejects_too_long_name() {
        let value = "a".repeat(101);
        let error = OrganizationName::try_from(value).expect_err("name should be too long");

        assert!(matches!(error, OrganizationNameError::TooLong));
    }
}
