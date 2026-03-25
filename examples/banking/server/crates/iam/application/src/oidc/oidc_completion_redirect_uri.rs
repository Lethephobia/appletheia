use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the redirect URI used after OIDC completion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OidcCompletionRedirectUri(Url);

impl OidcCompletionRedirectUri {
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl TryFrom<String> for OidcCompletionRedirectUri {
    type Error = url::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Url::parse(&value)?))
    }
}

impl From<OidcCompletionRedirectUri> for String {
    fn from(value: OidcCompletionRedirectUri) -> Self {
        value.0.into()
    }
}
