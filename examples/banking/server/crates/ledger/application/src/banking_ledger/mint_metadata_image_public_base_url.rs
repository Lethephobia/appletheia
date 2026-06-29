use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::CurrencyImageObjectName;
use serde::{Deserialize, Serialize};
use url::Url;

use super::{MintMetadataImagePublicBaseUrlError, MintMetadataImageUri};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MintMetadataImagePublicBaseUrl(Url);

impl MintMetadataImagePublicBaseUrl {
    pub fn new(mut value: Url) -> Result<Self, MintMetadataImagePublicBaseUrlError> {
        if !value.path().ends_with('/') {
            let path = format!("{}/", value.path());
            value.set_path(&path);
        }

        if value.cannot_be_a_base() {
            return Err(MintMetadataImagePublicBaseUrlError::InvalidBaseUrl);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &Url {
        &self.0
    }

    pub fn resolve(
        &self,
        object_name: &CurrencyImageObjectName,
    ) -> Result<MintMetadataImageUri, MintMetadataImagePublicBaseUrlError> {
        Ok(MintMetadataImageUri::new(self.0.join(object_name.value())?))
    }
}

impl Display for MintMetadataImagePublicBaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value().as_str())
    }
}

impl FromStr for MintMetadataImagePublicBaseUrl {
    type Err = MintMetadataImagePublicBaseUrlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(Url::parse(value)?)
    }
}

impl TryFrom<&str> for MintMetadataImagePublicBaseUrl {
    type Error = MintMetadataImagePublicBaseUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintMetadataImagePublicBaseUrl {
    type Error = MintMetadataImagePublicBaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<MintMetadataImagePublicBaseUrl> for String {
    fn from(value: MintMetadataImagePublicBaseUrl) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyImageObjectName;

    use super::MintMetadataImagePublicBaseUrl;

    #[test]
    fn normalizes_to_trailing_slash() {
        let base_url =
            MintMetadataImagePublicBaseUrl::try_from("https://assets.example.com/images")
                .expect("base URL should be valid");

        assert_eq!(
            base_url.value().as_str(),
            "https://assets.example.com/images/"
        );
    }

    #[test]
    fn resolves_object_name_under_base_url() {
        let base_url =
            MintMetadataImagePublicBaseUrl::try_from("https://assets.example.com/images/")
                .expect("base URL should be valid");
        let object_name = CurrencyImageObjectName::try_from(
            "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
        )
        .expect("object name should be valid");

        let uri = base_url.resolve(&object_name).expect("URI should resolve");

        assert_eq!(
            uri.value().as_str(),
            "https://assets.example.com/images/currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002"
        );
    }
}
