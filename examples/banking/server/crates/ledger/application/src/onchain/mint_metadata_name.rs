use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::CurrencyName;
use serde::{Deserialize, Serialize};

use super::MintMetadataNameError;

/// Represents the name included in off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintMetadataName(String);

impl MintMetadataName {
    /// Creates a mint metadata name.
    pub fn new(value: String) -> Result<Self, MintMetadataNameError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(MintMetadataNameError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the name value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MintMetadataName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintMetadataName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl From<&CurrencyName> for MintMetadataName {
    fn from(value: &CurrencyName) -> Self {
        Self(value.value().to_owned())
    }
}

impl FromStr for MintMetadataName {
    type Err = MintMetadataNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for MintMetadataName {
    type Error = MintMetadataNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintMetadataName {
    type Error = MintMetadataNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<MintMetadataName> for String {
    fn from(value: MintMetadataName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyName;

    use super::{MintMetadataName, MintMetadataNameError};

    #[test]
    fn accepts_valid_name() {
        let name = MintMetadataName::try_from("  USD Coin  ").expect("name should be valid");

        assert_eq!(name.value(), "USD Coin");
    }

    #[test]
    fn converts_from_currency_name() {
        let currency_name = CurrencyName::try_from("USD Coin").expect("name should be valid");

        let name = MintMetadataName::from(&currency_name);

        assert_eq!(name.value(), "USD Coin");
    }

    #[test]
    fn rejects_empty_name() {
        let error = MintMetadataName::try_from("   ").expect_err("empty name should fail");

        assert!(matches!(error, MintMetadataNameError::Empty));
    }
}
