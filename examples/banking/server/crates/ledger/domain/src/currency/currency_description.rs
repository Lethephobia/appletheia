use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyDescriptionError;

/// Represents a user-facing currency description.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyDescription(String);

impl CurrencyDescription {
    /// Creates a currency description from user input.
    pub fn new(value: String) -> Result<Self, CurrencyDescriptionError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(CurrencyDescriptionError::Empty);
        }

        if normalized.chars().count() > 280 {
            return Err(CurrencyDescriptionError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated description.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for CurrencyDescription {
    type Err = CurrencyDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyDescription {
    type Error = CurrencyDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyDescription {
    type Error = CurrencyDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyDescription> for String {
    fn from(value: CurrencyDescription) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencyDescription, CurrencyDescriptionError};

    #[test]
    fn accepts_valid_description() {
        let description = CurrencyDescription::try_from("  Stablecoin backed by USD  ")
            .expect("description should be valid");

        assert_eq!(description.value(), "Stablecoin backed by USD");
    }

    #[test]
    fn rejects_empty_description() {
        let error =
            CurrencyDescription::try_from("   ").expect_err("empty description should fail");

        assert!(matches!(error, CurrencyDescriptionError::Empty));
    }

    #[test]
    fn rejects_too_long_description() {
        let value = "a".repeat(281);
        let error =
            CurrencyDescription::try_from(value).expect_err("description should be too long");

        assert!(matches!(error, CurrencyDescriptionError::TooLong));
    }
}
