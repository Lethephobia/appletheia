use serde::{Deserialize, Serialize};
use url::Url;

use super::CurrencyImageUrlError;

/// Represents an externally hosted currency image URL.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyImageUrl(Url);

impl CurrencyImageUrl {
    /// Creates a new currency image URL.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the image URL value.
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl TryFrom<String> for CurrencyImageUrl {
    type Error = CurrencyImageUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Url::parse(&value)?))
    }
}

impl TryFrom<&str> for CurrencyImageUrl {
    type Error = CurrencyImageUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl From<CurrencyImageUrl> for String {
    fn from(value: CurrencyImageUrl) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyImageUrl;

    #[test]
    fn accepts_valid_image_url() {
        let image_url = CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
            .expect("image URL should be valid");

        assert_eq!(
            image_url.value().as_str(),
            "https://cdn.example.com/currencies/usdc.png"
        );
    }
}
