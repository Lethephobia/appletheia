use super::{AuthTokenExchangeCodeExpiresIn, PkceMode};

/// Configures default issuance policy for auth token exchange codes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeCodeIssuerConfig {
    expires_in: AuthTokenExchangeCodeExpiresIn,
    pkce_mode: PkceMode,
}

impl AuthTokenExchangeCodeIssuerConfig {
    /// Creates a new issuer config.
    pub fn new(expires_in: AuthTokenExchangeCodeExpiresIn, pkce_mode: PkceMode) -> Self {
        Self {
            expires_in,
            pkce_mode,
        }
    }

    /// Returns the default exchange code lifetime.
    pub fn expires_in(&self) -> AuthTokenExchangeCodeExpiresIn {
        self.expires_in
    }

    /// Returns the configured PKCE mode.
    pub fn pkce_mode(&self) -> PkceMode {
        self.pkce_mode
    }
}
