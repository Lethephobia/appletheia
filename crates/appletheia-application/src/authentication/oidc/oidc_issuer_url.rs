use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use super::OidcDiscoveryUrl;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcIssuerUrl(Url);

impl OidcIssuerUrl {
    pub fn new(value: Url) -> Self {
        let value = Self::normalize_trailing_slash(value);
        Self(value)
    }

    pub fn value(&self) -> &Url {
        &self.0
    }

    pub fn discovery_url(&self) -> OidcDiscoveryUrl {
        let mut url = self.0.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .expect("OidcIssuerUrl must be a base URL");
            segments.pop_if_empty();
            segments.push(".well-known");
            segments.push("openid-configuration");
        }
        OidcDiscoveryUrl::new(url)
    }

    fn normalize_trailing_slash(mut url: Url) -> Url {
        let path = url.path().to_string();
        if path != "/" && path.ends_with('/') {
            let trimmed = path.trim_end_matches('/');
            url.set_path(trimmed);
        }
        url
    }
}

impl fmt::Display for OidcIssuerUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl FromStr for OidcIssuerUrl {
    type Err = url::ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value)?;
        Ok(Self::new(url))
    }
}

impl TryFrom<String> for OidcIssuerUrl {
    type Error = url::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}
