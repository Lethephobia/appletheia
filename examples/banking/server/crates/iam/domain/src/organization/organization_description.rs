use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::OrganizationDescriptionError;

/// Represents a user-facing organization description.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OrganizationDescription(String);

impl OrganizationDescription {
    /// Creates a new organization description.
    pub fn new(value: String) -> Result<Self, OrganizationDescriptionError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrganizationDescriptionError::Empty);
        }

        if trimmed.chars().count() > 280 {
            return Err(OrganizationDescriptionError::TooLong);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the description value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OrganizationDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OrganizationDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OrganizationDescription {
    type Err = OrganizationDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for OrganizationDescription {
    type Error = OrganizationDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for OrganizationDescription {
    type Error = OrganizationDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OrganizationDescription> for String {
    fn from(value: OrganizationDescription) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationDescription, OrganizationDescriptionError};

    #[test]
    fn accepts_valid_description() {
        let description = OrganizationDescription::try_from("  Independent research lab  ")
            .expect("description should be valid");

        assert_eq!(description.value(), "Independent research lab");
    }

    #[test]
    fn rejects_empty_description() {
        let error =
            OrganizationDescription::try_from("   ").expect_err("empty description should fail");

        assert!(matches!(error, OrganizationDescriptionError::Empty));
    }

    #[test]
    fn rejects_too_long_description() {
        let value = "a".repeat(281);
        let error =
            OrganizationDescription::try_from(value).expect_err("description should be too long");

        assert!(matches!(error, OrganizationDescriptionError::TooLong));
    }
}
