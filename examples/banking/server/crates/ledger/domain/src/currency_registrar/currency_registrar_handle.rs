use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyRegistrarHandleError;

/// Represents a validated CurrencyRegistrar handle.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyRegistrarHandle(String);

impl CurrencyRegistrarHandle {
    pub fn new(value: String) -> Result<Self, CurrencyRegistrarHandleError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(CurrencyRegistrarHandleError::Empty);
        }
        if normalized.chars().count() > 64 {
            return Err(CurrencyRegistrarHandleError::TooLong);
        }
        if !normalized.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '_'
                || character == '-'
        }) {
            return Err(CurrencyRegistrarHandleError::InvalidCharacter);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyRegistrarHandle {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyRegistrarHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

impl FromStr for CurrencyRegistrarHandle {
    type Err = CurrencyRegistrarHandleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyRegistrarHandle {
    type Error = CurrencyRegistrarHandleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyRegistrarHandle {
    type Error = CurrencyRegistrarHandleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyRegistrarHandle> for String {
    fn from(value: CurrencyRegistrarHandle) -> Self {
        value.0
    }
}
