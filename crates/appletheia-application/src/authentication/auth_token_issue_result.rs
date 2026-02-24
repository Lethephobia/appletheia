use super::{AuthToken, AuthTokenClaims};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AuthTokenIssueResult {
    token: AuthToken,
    claims: AuthTokenClaims,
}

impl AuthTokenIssueResult {
    pub fn new(token: AuthToken, claims: AuthTokenClaims) -> Self {
        Self { token, claims }
    }

    pub fn token(&self) -> &AuthToken {
        &self.token
    }

    pub fn claims(&self) -> &AuthTokenClaims {
        &self.claims
    }
}
