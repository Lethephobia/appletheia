use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::OrganizationDisplayNameError;

/// Represents a validated organization display name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationDisplayName(String);

impl OrganizationDisplayName {
    /// Creates an organization display name from user input.
    pub fn new(value: String) -> Result<Self, OrganizationDisplayNameError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(OrganizationDisplayNameError::Empty);
        }

        if normalized.len() > 100 {
            return Err(OrganizationDisplayNameError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated display name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OrganizationDisplayName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OrganizationDisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OrganizationDisplayName {
    type Err = OrganizationDisplayNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for OrganizationDisplayName {
    type Error = OrganizationDisplayNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for OrganizationDisplayName {
    type Error = OrganizationDisplayNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationDisplayName, OrganizationDisplayNameError};

    #[test]
    fn accepts_valid_display_name() {
        let name = OrganizationDisplayName::try_from("  Acme Labs  ")
            .expect("display name should be valid");

        assert_eq!(name.value(), "Acme Labs");
    }

    #[test]
    fn rejects_empty_display_name() {
        let error =
            OrganizationDisplayName::try_from("   ").expect_err("empty display name should fail");

        assert!(matches!(error, OrganizationDisplayNameError::Empty));
    }
}
