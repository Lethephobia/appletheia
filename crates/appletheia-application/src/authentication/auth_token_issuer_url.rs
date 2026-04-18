use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use super::AuthTokenIssuerUrlError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenIssuerUrl(Url);

impl AuthTokenIssuerUrl {
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for AuthTokenIssuerUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for AuthTokenIssuerUrl {
    type Err = AuthTokenIssuerUrlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value).map_err(AuthTokenIssuerUrlError::Parse)?;
        Ok(Self(url))
    }
}

impl TryFrom<&str> for AuthTokenIssuerUrl {
    type Error = AuthTokenIssuerUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for AuthTokenIssuerUrl {
    type Error = AuthTokenIssuerUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_rejects_invalid_url() {
        assert!(matches!(
            "not a url".parse::<AuthTokenIssuerUrl>(),
            Err(AuthTokenIssuerUrlError::Parse(_))
        ));
    }

    #[test]
    fn from_str_accepts_valid_url() {
        let issuer_url = "https://example.com/issuer"
            .parse::<AuthTokenIssuerUrl>()
            .expect("valid issuer url");
        assert_eq!(issuer_url.value().as_str(), "https://example.com/issuer");
    }
}
