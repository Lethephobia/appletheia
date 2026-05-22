use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use url::Url;

use super::MintMetadataUriError;

/// Represents an off-chain mint metadata URI returned by a metadata publisher.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MintMetadataUri(Url);

impl MintMetadataUri {
    /// Creates a mint metadata URI.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the metadata URI value.
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl Display for MintMetadataUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value().as_str())
    }
}

impl TryFrom<String> for MintMetadataUri {
    type Error = MintMetadataUriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Url::parse(&value)?))
    }
}

impl TryFrom<&str> for MintMetadataUri {
    type Error = MintMetadataUriError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl From<MintMetadataUri> for String {
    fn from(value: MintMetadataUri) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::MintMetadataUri;

    #[test]
    fn accepts_valid_metadata_uri() {
        let uri = MintMetadataUri::try_from("https://storage.example.com/currencies/usdc.json")
            .expect("URI should be valid");

        assert_eq!(
            uri.value().as_str(),
            "https://storage.example.com/currencies/usdc.json"
        );
    }
}
