use appletheia::application::authentication::oidc::OidcTokens;
use appletheia::application::authentication::{
    AuthToken, AuthTokenExchangeCode, AuthTokenExchangeCodeExpiresAt, AuthTokenExpiresIn,
};

use crate::oidc::OidcCompletionRedirectUri;

/// Represents the result returned after completing an OIDC flow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OidcCompleteOutput {
    Token {
        completion_redirect_uri: OidcCompletionRedirectUri,
        auth_token: AuthToken,
        auth_token_expires_in: AuthTokenExpiresIn,
        oidc_tokens: OidcTokens,
    },
    ExchangeCode {
        completion_redirect_uri: OidcCompletionRedirectUri,
        auth_token_exchange_code: AuthTokenExchangeCode,
        auth_token_exchange_code_expires_at: AuthTokenExchangeCodeExpiresAt,
    },
    IdentityLinked {
        completion_redirect_uri: OidcCompletionRedirectUri,
        oidc_tokens: OidcTokens,
    },
}
