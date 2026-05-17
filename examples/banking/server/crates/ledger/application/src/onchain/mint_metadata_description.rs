use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::CurrencyDescription;
use serde::{Deserialize, Serialize};

use super::MintMetadataDescriptionError;

/// Represents the description included in off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintMetadataDescription(String);

impl MintMetadataDescription {
    /// Creates a mint metadata description.
    pub fn new(value: String) -> Result<Self, MintMetadataDescriptionError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(MintMetadataDescriptionError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the description value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MintMetadataDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintMetadataDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl From<&CurrencyDescription> for MintMetadataDescription {
    fn from(value: &CurrencyDescription) -> Self {
        Self(value.value().to_owned())
    }
}

impl FromStr for MintMetadataDescription {
    type Err = MintMetadataDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for MintMetadataDescription {
    type Error = MintMetadataDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintMetadataDescription {
    type Error = MintMetadataDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<MintMetadataDescription> for String {
    fn from(value: MintMetadataDescription) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyDescription;

    use super::{MintMetadataDescription, MintMetadataDescriptionError};

    #[test]
    fn accepts_valid_description() {
        let description = MintMetadataDescription::try_from("  Stablecoin backed by USD  ")
            .expect("description should be valid");

        assert_eq!(description.value(), "Stablecoin backed by USD");
    }

    #[test]
    fn converts_from_currency_description() {
        let currency_description = CurrencyDescription::try_from("Stablecoin backed by USD")
            .expect("currency description should be valid");

        let description = MintMetadataDescription::from(&currency_description);

        assert_eq!(description.value(), "Stablecoin backed by USD");
    }

    #[test]
    fn rejects_empty_description() {
        let error =
            MintMetadataDescription::try_from("   ").expect_err("empty description should fail");

        assert!(matches!(error, MintMetadataDescriptionError::Empty));
    }
}
