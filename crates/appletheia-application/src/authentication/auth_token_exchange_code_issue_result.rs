use super::{AuthTokenExchangeCode, AuthTokenExchangeCodeExpiresAt};

/// Returns the issued exchange code and its expiration timestamp.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeCodeIssueResult {
    code: AuthTokenExchangeCode,
    expires_at: AuthTokenExchangeCodeExpiresAt,
}

impl AuthTokenExchangeCodeIssueResult {
    /// Creates a new exchange code issue result.
    pub fn new(code: AuthTokenExchangeCode, expires_at: AuthTokenExchangeCodeExpiresAt) -> Self {
        Self { code, expires_at }
    }

    /// Returns the issued exchange code.
    pub fn code(&self) -> &AuthTokenExchangeCode {
        &self.code
    }

    /// Returns when the issued exchange code expires.
    pub fn expires_at(&self) -> AuthTokenExchangeCodeExpiresAt {
        self.expires_at
    }
}
