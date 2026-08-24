use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyRegistrarDescriptionError;

/// Represents a validated CurrencyRegistrar description.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyRegistrarDescription(String);

impl CurrencyRegistrarDescription {
    pub fn new(value: String) -> Result<Self, CurrencyRegistrarDescriptionError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(CurrencyRegistrarDescriptionError::Empty);
        }
        if normalized.chars().count() > 280 {
            return Err(CurrencyRegistrarDescriptionError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyRegistrarDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyRegistrarDescription {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

impl FromStr for CurrencyRegistrarDescription {
    type Err = CurrencyRegistrarDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyRegistrarDescription {
    type Error = CurrencyRegistrarDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyRegistrarDescription {
    type Error = CurrencyRegistrarDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyRegistrarDescription> for String {
    fn from(value: CurrencyRegistrarDescription) -> Self {
        value.0
    }
}
