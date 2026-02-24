use std::fmt;

use serde::{Deserialize, Serialize};
use url::Url;

use super::AuthTokenIssuerUrlError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenIssuerUrl(Url);

impl AuthTokenIssuerUrl {
    pub fn parse(value: &str) -> Result<Self, AuthTokenIssuerUrlError> {
        let url = Url::parse(value).map_err(AuthTokenIssuerUrlError::Parse)?;
        Ok(Self(url))
    }

    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for AuthTokenIssuerUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rejects_invalid_url() {
        assert!(matches!(
            AuthTokenIssuerUrl::parse("not a url"),
            Err(AuthTokenIssuerUrlError::Parse(_))
        ));
    }

    #[test]
    fn parse_accepts_valid_url() {
        let issuer_url = AuthTokenIssuerUrl::parse("https://example.com/issuer").unwrap();
        assert_eq!(issuer_url.value().as_str(), "https://example.com/issuer");
    }
}
