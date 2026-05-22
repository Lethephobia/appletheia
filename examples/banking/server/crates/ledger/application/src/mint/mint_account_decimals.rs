use std::fmt::{self, Display};

use banking_ledger_domain::currency::CurrencyDecimals;
use serde::{Deserialize, Serialize};

/// Represents the decimal precision used when creating an on-chain mint account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintAccountDecimals(u8);

impl MintAccountDecimals {
    /// Creates mint account decimals from an integer precision.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the decimal precision.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for MintAccountDecimals {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<CurrencyDecimals> for MintAccountDecimals {
    fn from(value: CurrencyDecimals) -> Self {
        Self::new(value.value())
    }
}

impl From<&CurrencyDecimals> for MintAccountDecimals {
    fn from(value: &CurrencyDecimals) -> Self {
        Self::new(value.value())
    }
}

impl From<MintAccountDecimals> for u8 {
    fn from(value: MintAccountDecimals) -> Self {
        value.value()
    }
}

impl Display for MintAccountDecimals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyDecimals;

    use super::MintAccountDecimals;

    #[test]
    fn stores_decimal_precision() {
        let decimals = MintAccountDecimals::new(9);

        assert_eq!(decimals.value(), 9);
    }

    #[test]
    fn converts_from_currency_decimals() {
        let decimals = MintAccountDecimals::from(CurrencyDecimals::new(6));

        assert_eq!(decimals.value(), 6);
    }
}
