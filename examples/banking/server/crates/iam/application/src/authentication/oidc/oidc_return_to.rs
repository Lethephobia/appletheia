use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::{Position, Url};

use super::OidcReturnToError;

const VALIDATION_ORIGIN: &str = "https://oidc-return-to.invalid";
const VALIDATION_HOST: &str = "oidc-return-to.invalid";

/// Represents an application-local destination used after OIDC completion.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OidcReturnTo(String);

impl OidcReturnTo {
    /// Creates an OIDC return destination from an application-local absolute path.
    pub fn new(value: String) -> Result<Self, OidcReturnToError> {
        if !value.starts_with('/')
            || value.starts_with("//")
            || value.contains('\\')
            || value.chars().any(char::is_control)
        {
            return Err(OidcReturnToError::NotApplicationLocal);
        }

        let validation_url = format!("{VALIDATION_ORIGIN}{value}");
        let parsed_url =
            Url::parse(&validation_url).map_err(|_| OidcReturnToError::NotApplicationLocal)?;

        if parsed_url.scheme() != "https"
            || parsed_url.host_str() != Some(VALIDATION_HOST)
            || parsed_url.port().is_some()
            || !parsed_url.username().is_empty()
            || parsed_url.password().is_some()
        {
            return Err(OidcReturnToError::NotApplicationLocal);
        }

        let normalized_value = parsed_url[Position::BeforePath..].to_owned();
        if normalized_value.starts_with("//") {
            return Err(OidcReturnToError::NotApplicationLocal);
        }

        Ok(Self(normalized_value))
    }

    /// Returns the application-local destination.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OidcReturnTo {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OidcReturnTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OidcReturnTo {
    type Err = OidcReturnToError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl TryFrom<&str> for OidcReturnTo {
    type Error = OidcReturnToError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for OidcReturnTo {
    type Error = OidcReturnToError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OidcReturnTo> for String {
    fn from(value: OidcReturnTo) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{OidcReturnTo, OidcReturnToError};

    #[test]
    fn accepts_an_application_local_destination() {
        let return_to = OidcReturnTo::try_from("/organizations/123?tab=members#pending")
            .expect("application-local destination should be valid");

        assert_eq!(return_to.value(), "/organizations/123?tab=members#pending");
    }

    #[test]
    fn normalizes_path_segments() {
        let return_to = OidcReturnTo::try_from("/organizations/../wallets")
            .expect("application-local destination should be valid");

        assert_eq!(return_to.value(), "/wallets");
    }

    #[test]
    fn rejects_an_absolute_url() {
        let error = OidcReturnTo::try_from("https://evil.example/path")
            .expect_err("absolute URL should be rejected");

        assert_eq!(error, OidcReturnToError::NotApplicationLocal);
    }

    #[test]
    fn rejects_a_network_path_reference() {
        let error = OidcReturnTo::try_from("//evil.example/path")
            .expect_err("network-path reference should be rejected");

        assert_eq!(error, OidcReturnToError::NotApplicationLocal);
    }

    #[test]
    fn rejects_a_backslash_based_network_path_reference() {
        let error = OidcReturnTo::try_from("/\\evil.example/path")
            .expect_err("backslash-based network-path reference should be rejected");

        assert_eq!(error, OidcReturnToError::NotApplicationLocal);
    }

    #[test]
    fn rejects_a_network_path_reference_created_by_normalization() {
        let error = OidcReturnTo::try_from("/organizations/..//evil.example/path")
            .expect_err("normalized network-path reference should be rejected");

        assert_eq!(error, OidcReturnToError::NotApplicationLocal);
    }

    #[test]
    fn rejects_a_relative_path() {
        let error = OidcReturnTo::try_from("organizations/123")
            .expect_err("relative path should be rejected");

        assert_eq!(error, OidcReturnToError::NotApplicationLocal);
    }
}
