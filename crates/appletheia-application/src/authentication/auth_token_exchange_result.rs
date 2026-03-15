use super::{AuthTokenIssueResult, oidc::OidcTokens};

/// Returns the values produced when an exchange code is redeemed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeResult {
    auth_token_issue_result: AuthTokenIssueResult,
    oidc_tokens: Option<OidcTokens>,
}

impl AuthTokenExchangeResult {
    /// Creates a new exchange result.
    pub fn new(
        auth_token_issue_result: AuthTokenIssueResult,
        oidc_tokens: Option<OidcTokens>,
    ) -> Self {
        Self {
            auth_token_issue_result,
            oidc_tokens,
        }
    }

    /// Returns the issued auth token result.
    pub fn auth_token_issue_result(&self) -> &AuthTokenIssueResult {
        &self.auth_token_issue_result
    }

    /// Returns OIDC tokens recovered from the exchange payload, if any.
    pub fn oidc_tokens(&self) -> Option<&OidcTokens> {
        self.oidc_tokens.as_ref()
    }
}
