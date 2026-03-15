use appletheia_application::authentication::oidc::{
    OidcAccessToken, OidcIdToken, OidcRefreshToken, OidcTokenExpiresIn, OidcTokens,
};
use serde::{Deserialize, Serialize};

/// Represents OIDC tokens in the JSON form used by the grant cipher.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AuthTokenExchangeOidcTokensJson {
    pub id_token: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in_seconds: Option<u64>,
}

impl AuthTokenExchangeOidcTokensJson {
    pub(crate) fn from_oidc_tokens(tokens: &OidcTokens) -> Self {
        Self {
            id_token: tokens.id_token().value().to_owned(),
            access_token: tokens.access_token().map(|value| value.value().to_owned()),
            refresh_token: tokens.refresh_token().map(|value| value.value().to_owned()),
            expires_in_seconds: tokens.expires_in().and_then(|value| {
                let seconds = value.value().num_seconds();
                u64::try_from(seconds).ok()
            }),
        }
    }

    pub(crate) fn into_oidc_tokens(
        self,
    ) -> Result<OidcTokens, Box<dyn std::error::Error + Send + Sync>> {
        let expires_in = self
            .expires_in_seconds
            .map(OidcTokenExpiresIn::from_seconds)
            .transpose()?;

        Ok(OidcTokens::new(
            OidcIdToken::new(self.id_token),
            self.access_token.map(OidcAccessToken::new),
            self.refresh_token.map(OidcRefreshToken::new),
            expires_in,
        ))
    }
}
