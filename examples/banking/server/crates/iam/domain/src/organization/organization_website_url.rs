use serde::{Deserialize, Serialize};
use url::Url;

use super::OrganizationWebsiteUrlError;

/// Represents an organization's public website URL.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OrganizationWebsiteUrl(Url);

impl OrganizationWebsiteUrl {
    /// Creates a new organization website URL.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the website URL value.
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl TryFrom<String> for OrganizationWebsiteUrl {
    type Error = OrganizationWebsiteUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Url::parse(&value)?))
    }
}

impl TryFrom<&str> for OrganizationWebsiteUrl {
    type Error = OrganizationWebsiteUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl From<OrganizationWebsiteUrl> for String {
    fn from(value: OrganizationWebsiteUrl) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationWebsiteUrl;

    #[test]
    fn accepts_valid_website_url() {
        let website_url = OrganizationWebsiteUrl::try_from("https://acme.example.com")
            .expect("website URL should be valid");

        assert_eq!(website_url.value().as_str(), "https://acme.example.com/");
    }

    #[test]
    fn rejects_invalid_website_url() {
        let error =
            OrganizationWebsiteUrl::try_from("not a url").expect_err("invalid URL should fail");

        assert!(matches!(
            error,
            super::OrganizationWebsiteUrlError::Parse(_)
        ));
    }
}
