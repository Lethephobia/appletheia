use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::CurrencyImageObjectName;
use serde::{Deserialize, Serialize};

use super::MintMetadataImageObjectNameError;

/// Represents an object storage name for an image included in off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintMetadataImageObjectName(String);

impl MintMetadataImageObjectName {
    /// Creates a mint metadata image object name.
    pub fn new(value: String) -> Result<Self, MintMetadataImageObjectNameError> {
        if value.is_empty() {
            return Err(MintMetadataImageObjectNameError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the image object name value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MintMetadataImageObjectName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintMetadataImageObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl From<&CurrencyImageObjectName> for MintMetadataImageObjectName {
    fn from(value: &CurrencyImageObjectName) -> Self {
        Self(value.value().to_owned())
    }
}

impl FromStr for MintMetadataImageObjectName {
    type Err = MintMetadataImageObjectNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for MintMetadataImageObjectName {
    type Error = MintMetadataImageObjectNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintMetadataImageObjectName {
    type Error = MintMetadataImageObjectNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<MintMetadataImageObjectName> for String {
    fn from(value: MintMetadataImageObjectName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::{CurrencyId, CurrencyImageObjectName};

    use super::{MintMetadataImageObjectName, MintMetadataImageObjectNameError};

    #[test]
    fn accepts_valid_image_object_name() {
        let object_name = MintMetadataImageObjectName::try_from("currencies/test/images/image")
            .expect("object name should be valid");

        assert_eq!(object_name.value(), "currencies/test/images/image");
    }

    #[test]
    fn converts_from_currency_image_object_name() {
        let currency_object_name = CurrencyImageObjectName::new(CurrencyId::new());

        let object_name = MintMetadataImageObjectName::from(&currency_object_name);

        assert_eq!(object_name.value(), currency_object_name.value());
    }

    #[test]
    fn rejects_empty_image_object_name() {
        let error = MintMetadataImageObjectName::try_from("").expect_err("empty name should fail");

        assert!(matches!(error, MintMetadataImageObjectNameError::Empty));
    }
}
