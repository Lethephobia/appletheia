use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{AuthTokenIssuerUrl, AuthTokenIssuerUrlsError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenIssuerUrls(Vec<AuthTokenIssuerUrl>);

impl AuthTokenIssuerUrls {
    pub fn new(mut issuer_urls: Vec<AuthTokenIssuerUrl>) -> Result<Self, AuthTokenIssuerUrlsError> {
        if issuer_urls.is_empty() {
            return Err(AuthTokenIssuerUrlsError::Empty);
        }

        let mut seen = HashSet::new();
        issuer_urls.retain(|issuer_url| seen.insert(issuer_url.clone()));

        Ok(Self(issuer_urls))
    }

    pub fn values(&self) -> &[AuthTokenIssuerUrl] {
        &self.0
    }
}

impl TryFrom<Vec<AuthTokenIssuerUrl>> for AuthTokenIssuerUrls {
    type Error = AuthTokenIssuerUrlsError;

    fn try_from(value: Vec<AuthTokenIssuerUrl>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::AuthTokenIssuerUrls;
    use crate::authentication::{AuthTokenIssuerUrl, AuthTokenIssuerUrlsError};

    #[test]
    fn new_rejects_empty() {
        assert_eq!(
            AuthTokenIssuerUrls::new(vec![]),
            Err(AuthTokenIssuerUrlsError::Empty)
        );
    }

    #[test]
    fn new_dedupes_issuer_urls() {
        let issuer_urls = vec![
            "https://issuer-a.example.com"
                .parse::<AuthTokenIssuerUrl>()
                .expect("issuer url should be valid"),
            "https://issuer-a.example.com"
                .parse::<AuthTokenIssuerUrl>()
                .expect("issuer url should be valid"),
            "https://issuer-b.example.com"
                .parse::<AuthTokenIssuerUrl>()
                .expect("issuer url should be valid"),
        ];

        let issuer_urls =
            AuthTokenIssuerUrls::new(issuer_urls).expect("issuer urls should be valid");
        let values: Vec<&str> = issuer_urls
            .values()
            .iter()
            .map(|issuer_url| issuer_url.value().as_str())
            .collect();

        assert_eq!(
            values,
            vec![
                "https://issuer-a.example.com/",
                "https://issuer-b.example.com/",
            ]
        );
    }
}
