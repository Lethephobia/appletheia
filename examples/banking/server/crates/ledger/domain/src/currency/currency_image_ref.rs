use serde::{Deserialize, Serialize};

use super::{CurrencyImageObjectName, CurrencyImageUrl};

/// Represents a currency image reference stored by the domain.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum CurrencyImageRef {
    ObjectName(CurrencyImageObjectName),
    ExternalUrl(CurrencyImageUrl),
}

impl CurrencyImageRef {
    /// Creates a currency image reference backed by object storage.
    pub fn object_name(object_name: CurrencyImageObjectName) -> Self {
        Self::ObjectName(object_name)
    }

    /// Creates a currency image reference backed by an external URL.
    pub fn external_url(url: CurrencyImageUrl) -> Self {
        Self::ExternalUrl(url)
    }

    /// Returns the object name when this image is stored in object storage.
    pub fn as_object_name(&self) -> Option<&CurrencyImageObjectName> {
        match self {
            Self::ObjectName(value) => Some(value),
            Self::ExternalUrl(_) => None,
        }
    }

    /// Returns the external URL when this image is hosted outside object storage.
    pub fn as_external_url(&self) -> Option<&CurrencyImageUrl> {
        match self {
            Self::ObjectName(_) => None,
            Self::ExternalUrl(value) => Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencyImageObjectName, CurrencyImageRef, CurrencyImageUrl};

    #[test]
    fn returns_object_name_when_present() {
        let image = CurrencyImageRef::object_name(
            CurrencyImageObjectName::try_from(
                "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
            )
            .expect("name should be valid"),
        );

        assert_eq!(
            image.as_object_name().map(CurrencyImageObjectName::value),
            Some(
                "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002"
            )
        );
        assert_eq!(image.as_external_url(), None);
    }

    #[test]
    fn returns_external_url_when_present() {
        let image = CurrencyImageRef::external_url(
            CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
                .expect("URL should be valid"),
        );

        assert_eq!(
            image.as_external_url().map(|value| value.value().as_str()),
            Some("https://cdn.example.com/currencies/usdc.png")
        );
        assert_eq!(image.as_object_name(), None);
    }
}
