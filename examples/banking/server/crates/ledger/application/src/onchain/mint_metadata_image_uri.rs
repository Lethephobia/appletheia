use std::fmt::{self, Display};

use banking_ledger_domain::currency::CurrencyImageUrl;
use serde::{Deserialize, Serialize};
use url::Url;

use super::MintMetadataImageUriError;

/// Represents an image URI included in off-chain mint metadata.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MintMetadataImageUri(Url);

impl MintMetadataImageUri {
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl Display for MintMetadataImageUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value().as_str())
    }
}

impl TryFrom<String> for MintMetadataImageUri {
    type Error = MintMetadataImageUriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::new(Url::parse(&value)?))
    }
}

impl TryFrom<CurrencyImageUrl> for MintMetadataImageUri {
    type Error = MintMetadataImageUriError;

    fn try_from(value: CurrencyImageUrl) -> Result<Self, Self::Error> {
        Ok(Self::new(value.value().clone()))
    }
}

impl TryFrom<&CurrencyImageUrl> for MintMetadataImageUri {
    type Error = MintMetadataImageUriError;

    fn try_from(value: &CurrencyImageUrl) -> Result<Self, Self::Error> {
        Ok(Self::new(value.value().clone()))
    }
}

impl From<MintMetadataImageUri> for String {
    fn from(value: MintMetadataImageUri) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyImageUrl;

    use super::MintMetadataImageUri;

    #[test]
    fn converts_from_currency_image_url() {
        let image_url = CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
            .expect("image URL should be valid");
        let uri = MintMetadataImageUri::try_from(&image_url).expect("URI should be valid");

        assert_eq!(
            uri.value().as_str(),
            "https://cdn.example.com/currencies/usdc.png"
        );
    }
}
