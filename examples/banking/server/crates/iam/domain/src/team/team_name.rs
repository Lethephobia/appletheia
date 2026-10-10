use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::TeamNameError;

/// Represents a validated team name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TeamName(String);

impl TeamName {
    /// Creates a team name from user input.
    pub fn new(value: String) -> Result<Self, TeamNameError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(TeamNameError::Empty);
        }

        if normalized.len() > 100 {
            return Err(TeamNameError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TeamName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for TeamName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for TeamName {
    type Err = TeamNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for TeamName {
    type Error = TeamNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for TeamName {
    type Error = TeamNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{TeamName, TeamNameError};

    #[test]
    fn accepts_valid_name() {
        let name = TeamName::try_from("  Acme Labs  ").expect("name should be valid");

        assert_eq!(name.value(), "Acme Labs");
    }

    #[test]
    fn rejects_empty_name() {
        let error = TeamName::try_from("   ").expect_err("empty name should fail");

        assert!(matches!(error, TeamNameError::Empty));
    }
}
