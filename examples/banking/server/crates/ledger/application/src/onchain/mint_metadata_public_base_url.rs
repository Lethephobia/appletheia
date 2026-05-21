use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::onchain::MintMetadataUri;

use super::{MintMetadataObjectName, MintMetadataPublicBaseUrlError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MintMetadataPublicBaseUrl(Url);

impl MintMetadataPublicBaseUrl {
    pub fn new(mut value: Url) -> Result<Self, MintMetadataPublicBaseUrlError> {
        if !value.path().ends_with('/') {
            let path = format!("{}/", value.path());
            value.set_path(&path);
        }

        if value.cannot_be_a_base() {
            return Err(MintMetadataPublicBaseUrlError::InvalidBaseUrl);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &Url {
        &self.0
    }

    pub fn resolve(
        &self,
        object_name: &MintMetadataObjectName,
    ) -> Result<MintMetadataUri, MintMetadataPublicBaseUrlError> {
        Ok(MintMetadataUri::new(self.0.join(object_name.value())?))
    }
}

impl Display for MintMetadataPublicBaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value().as_str())
    }
}

impl FromStr for MintMetadataPublicBaseUrl {
    type Err = MintMetadataPublicBaseUrlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(Url::parse(value)?)
    }
}

impl TryFrom<&str> for MintMetadataPublicBaseUrl {
    type Error = MintMetadataPublicBaseUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintMetadataPublicBaseUrl {
    type Error = MintMetadataPublicBaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<MintMetadataPublicBaseUrl> for String {
    fn from(value: MintMetadataPublicBaseUrl) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::{MintMetadataObjectName, MintMetadataPublicBaseUrl};

    #[test]
    fn normalizes_to_trailing_slash() {
        let base_url = MintMetadataPublicBaseUrl::try_from("https://storage.example.com/bucket")
            .expect("base URL should be valid");

        assert_eq!(
            base_url.value().as_str(),
            "https://storage.example.com/bucket/"
        );
    }

    #[test]
    fn resolves_object_name_under_base_url() {
        let base_url = MintMetadataPublicBaseUrl::try_from("https://storage.example.com/bucket/")
            .expect("base URL should be valid");
        let object_name = MintMetadataObjectName::try_from("currencies/metadata.json")
            .expect("object name should be valid");

        let uri = base_url.resolve(&object_name).expect("URI should resolve");

        assert_eq!(
            uri.value().as_str(),
            "https://storage.example.com/bucket/currencies/metadata.json"
        );
    }
}
