use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyDefinitionNameError;

/// Represents a validated currency-definition name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyDefinitionName(String);

impl CurrencyDefinitionName {
    /// Creates a currency-definition name from user input.
    pub fn new(value: String) -> Result<Self, CurrencyDefinitionNameError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(CurrencyDefinitionNameError::Empty);
        }

        if normalized.len() > 100 {
            return Err(CurrencyDefinitionNameError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyDefinitionName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyDefinitionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for CurrencyDefinitionName {
    type Err = CurrencyDefinitionNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyDefinitionName {
    type Error = CurrencyDefinitionNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyDefinitionName {
    type Error = CurrencyDefinitionNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencyDefinitionName, CurrencyDefinitionNameError};

    #[test]
    fn accepts_valid_name() {
        let name = CurrencyDefinitionName::try_from("  USD Coin  ").expect("name should be valid");

        assert_eq!(name.value(), "USD Coin");
    }

    #[test]
    fn rejects_empty_name() {
        let error = CurrencyDefinitionName::try_from("   ").expect_err("empty name should fail");

        assert!(matches!(error, CurrencyDefinitionNameError::Empty));
    }

    #[test]
    fn rejects_too_long_name() {
        let value = "a".repeat(101);
        let error = CurrencyDefinitionName::try_from(value).expect_err("name should be too long");

        assert!(matches!(error, CurrencyDefinitionNameError::TooLong));
    }
}
