use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyCodeError;

/// Represents a validated Ledger currency code.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyCode(String);

impl CurrencyCode {
    /// Creates a currency code.
    pub fn new(value: String) -> Result<Self, CurrencyCodeError> {
        let value = value.trim().to_owned();
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_uppercase()) {
            return Err(CurrencyCodeError::InvalidFormat);
        }
        Ok(Self(value))
    }

    /// Returns the currency code value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for CurrencyCode {
    type Err = CurrencyCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = CurrencyCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyCode {
    type Error = CurrencyCodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyCode> for String {
    fn from(value: CurrencyCode) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencyCode, CurrencyCodeError};

    #[test]
    fn accepts_variable_length_uppercase_ascii_letters() {
        for value in ["A", "USD", "USDT", "AAPL", "APPLETHEIA"] {
            let code = CurrencyCode::try_from(value).expect("currency code should be valid");

            assert_eq!(code.value(), value);
        }
    }

    #[test]
    fn rejects_noncanonical_codes() {
        for value in ["", "usd", "U1D", "US-D"] {
            assert_eq!(
                CurrencyCode::try_from(value).expect_err("currency code should be invalid"),
                CurrencyCodeError::InvalidFormat
            );
        }
    }
}
