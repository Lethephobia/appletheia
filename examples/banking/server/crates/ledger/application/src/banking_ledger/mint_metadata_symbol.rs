use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::CurrencySymbol;
use serde::{Deserialize, Serialize};

use super::MintMetadataSymbolError;

/// Represents the symbol included in off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintMetadataSymbol(String);

impl MintMetadataSymbol {
    /// Creates a mint metadata symbol.
    pub fn new(value: String) -> Result<Self, MintMetadataSymbolError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(MintMetadataSymbolError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the symbol value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MintMetadataSymbol {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintMetadataSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl From<&CurrencySymbol> for MintMetadataSymbol {
    fn from(value: &CurrencySymbol) -> Self {
        Self(value.value().to_owned())
    }
}

impl FromStr for MintMetadataSymbol {
    type Err = MintMetadataSymbolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for MintMetadataSymbol {
    type Error = MintMetadataSymbolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintMetadataSymbol {
    type Error = MintMetadataSymbolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<MintMetadataSymbol> for String {
    fn from(value: MintMetadataSymbol) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencySymbol;

    use super::{MintMetadataSymbol, MintMetadataSymbolError};

    #[test]
    fn accepts_valid_symbol() {
        let symbol = MintMetadataSymbol::try_from("  USDC  ").expect("symbol should be valid");

        assert_eq!(symbol.value(), "USDC");
    }

    #[test]
    fn converts_from_currency_symbol() {
        let currency_symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");

        let symbol = MintMetadataSymbol::from(&currency_symbol);

        assert_eq!(symbol.value(), "USDC");
    }

    #[test]
    fn rejects_empty_symbol() {
        let error = MintMetadataSymbol::try_from("   ").expect_err("empty symbol should fail");

        assert!(matches!(error, MintMetadataSymbolError::Empty));
    }
}
