use crate::authorization::AggregateRef;

use super::oidc::OidcTokens;

/// Describes what should be returned when an exchange code is redeemed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeGrant {
    subject: AggregateRef,
    oidc_tokens: Option<OidcTokens>,
}

impl AuthTokenExchangeGrant {
    /// Creates a new exchange grant for a subject.
    pub fn new(subject: AggregateRef, oidc_tokens: Option<OidcTokens>) -> Self {
        Self {
            subject,
            oidc_tokens,
        }
    }

    /// Returns the authenticated subject.
    pub fn subject(&self) -> &AggregateRef {
        &self.subject
    }

    /// Returns the OIDC tokens to return, if any.
    pub fn oidc_tokens(&self) -> Option<&OidcTokens> {
        self.oidc_tokens.as_ref()
    }
}
