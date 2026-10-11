use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::TeamDescriptionError;

/// Represents a user-facing team description.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TeamDescription(String);

impl TeamDescription {
    /// Creates a new team description.
    pub fn new(value: String) -> Result<Self, TeamDescriptionError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(TeamDescriptionError::Empty);
        }

        if trimmed.chars().count() > 280 {
            return Err(TeamDescriptionError::TooLong);
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the description value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TeamDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for TeamDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for TeamDescription {
    type Err = TeamDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for TeamDescription {
    type Error = TeamDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for TeamDescription {
    type Error = TeamDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<TeamDescription> for String {
    fn from(value: TeamDescription) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{TeamDescription, TeamDescriptionError};

    #[test]
    fn accepts_valid_description() {
        let description = TeamDescription::try_from("  Independent research lab  ")
            .expect("description should be valid");

        assert_eq!(description.value(), "Independent research lab");
    }

    #[test]
    fn rejects_empty_description() {
        let error = TeamDescription::try_from("   ").expect_err("empty description should fail");

        assert!(matches!(error, TeamDescriptionError::Empty));
    }

    #[test]
    fn rejects_too_long_description() {
        let value = "a".repeat(281);
        let error = TeamDescription::try_from(value).expect_err("description should be too long");

        assert!(matches!(error, TeamDescriptionError::TooLong));
    }
}
