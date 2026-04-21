use serde::{Deserialize, Serialize};
use url::Url;

use super::OrganizationPictureUrlError;

/// Represents an externally hosted organization picture URL.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OrganizationPictureUrl(Url);

impl OrganizationPictureUrl {
    /// Creates a new organization picture URL.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the picture URL value.
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl TryFrom<String> for OrganizationPictureUrl {
    type Error = OrganizationPictureUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Url::parse(&value)?))
    }
}

impl TryFrom<&str> for OrganizationPictureUrl {
    type Error = OrganizationPictureUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl From<OrganizationPictureUrl> for String {
    fn from(value: OrganizationPictureUrl) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationPictureUrl;

    #[test]
    fn accepts_valid_picture_url() {
        let picture_url = OrganizationPictureUrl::try_from("https://cdn.example.com/acme.png")
            .expect("picture URL should be valid");

        assert_eq!(
            picture_url.value().as_str(),
            "https://cdn.example.com/acme.png"
        );
    }
}
