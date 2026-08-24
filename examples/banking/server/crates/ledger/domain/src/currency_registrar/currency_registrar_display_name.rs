use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyRegistrarDisplayNameError;

/// Represents a validated CurrencyRegistrar display name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyRegistrarDisplayName(String);

impl CurrencyRegistrarDisplayName {
    pub fn new(value: String) -> Result<Self, CurrencyRegistrarDisplayNameError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(CurrencyRegistrarDisplayNameError::Empty);
        }
        if normalized.chars().count() > 100 {
            return Err(CurrencyRegistrarDisplayNameError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyRegistrarDisplayName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyRegistrarDisplayName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

impl FromStr for CurrencyRegistrarDisplayName {
    type Err = CurrencyRegistrarDisplayNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyRegistrarDisplayName {
    type Error = CurrencyRegistrarDisplayNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyRegistrarDisplayName {
    type Error = CurrencyRegistrarDisplayNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyRegistrarDisplayName> for String {
    fn from(value: CurrencyRegistrarDisplayName) -> Self {
        value.0
    }
}
