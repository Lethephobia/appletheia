use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Represents the decimal precision of a currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyDecimals(u8);

impl CurrencyDecimals {
    /// Creates currency decimals from an integer precision.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the decimal precision.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for CurrencyDecimals {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl Display for CurrencyDecimals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyDecimals;

    #[test]
    fn stores_decimal_precision() {
        let decimals = CurrencyDecimals::new(9);

        assert_eq!(decimals.value(), 9);
    }
}
