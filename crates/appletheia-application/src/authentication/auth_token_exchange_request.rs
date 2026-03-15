use super::{AuthTokenExchangeCode, PkceCodeVerifier};

/// Contains the information required to redeem an auth token exchange code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeRequest {
    code: AuthTokenExchangeCode,
    code_verifier: Option<PkceCodeVerifier>,
}

impl AuthTokenExchangeRequest {
    /// Creates a new exchange request.
    pub fn new(code: AuthTokenExchangeCode, code_verifier: Option<PkceCodeVerifier>) -> Self {
        Self {
            code,
            code_verifier,
        }
    }

    /// Returns the exchange code to redeem.
    pub fn code(&self) -> &AuthTokenExchangeCode {
        &self.code
    }

    /// Returns the code verifier, if present.
    pub fn code_verifier(&self) -> Option<&PkceCodeVerifier> {
        self.code_verifier.as_ref()
    }
}
