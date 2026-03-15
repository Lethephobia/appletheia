use super::{AuthTokenExchangeGrant, PkceCodeChallenge};

/// Contains the information required to issue an exchange code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeCodeIssueRequest {
    grant: AuthTokenExchangeGrant,
    code_challenge: Option<PkceCodeChallenge>,
}

impl AuthTokenExchangeCodeIssueRequest {
    /// Creates a new exchange code issue request.
    pub fn new(grant: AuthTokenExchangeGrant, code_challenge: Option<PkceCodeChallenge>) -> Self {
        Self {
            grant,
            code_challenge,
        }
    }

    /// Returns the exchange grant to embed in the code.
    pub fn grant(&self) -> &AuthTokenExchangeGrant {
        &self.grant
    }

    /// Returns the optional code challenge.
    pub fn code_challenge(&self) -> Option<&PkceCodeChallenge> {
        self.code_challenge.as_ref()
    }
}
