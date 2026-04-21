use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencySymbolError;

/// Represents a validated currency symbol.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencySymbol(String);

impl CurrencySymbol {
    /// Creates a currency symbol from user input.
    pub fn new(value: String) -> Result<Self, CurrencySymbolError> {
        let normalized = value.trim().to_ascii_uppercase();

        if normalized.is_empty() {
            return Err(CurrencySymbolError::Empty);
        }

        if normalized.len() > 10 {
            return Err(CurrencySymbolError::TooLong);
        }

        if !normalized.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            return Err(CurrencySymbolError::InvalidFormat);
        }

        Ok(Self(normalized))
    }

    /// Returns the validated currency symbol.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencySymbol {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencySymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for CurrencySymbol {
    type Err = CurrencySymbolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencySymbol {
    type Error = CurrencySymbolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencySymbol {
    type Error = CurrencySymbolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencySymbol, CurrencySymbolError};

    #[test]
    fn accepts_valid_symbol() {
        let symbol = CurrencySymbol::try_from(" usdc ").expect("symbol should be valid");

        assert_eq!(symbol.value(), "USDC");
    }

    #[test]
    fn rejects_empty_symbol() {
        let error = CurrencySymbol::try_from("   ").expect_err("empty symbol should fail");

        assert!(matches!(error, CurrencySymbolError::Empty));
    }

    #[test]
    fn rejects_too_long_symbol() {
        let error = CurrencySymbol::try_from("ABCDEFGHIJK".to_owned())
            .expect_err("too long symbol should fail");

        assert!(matches!(error, CurrencySymbolError::TooLong));
    }

    #[test]
    fn rejects_invalid_symbol() {
        let error = CurrencySymbol::try_from("usd-coin").expect_err("invalid symbol should fail");

        assert!(matches!(error, CurrencySymbolError::InvalidFormat));
    }
}
