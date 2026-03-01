use chrono::{DateTime, Utc};

use super::{OidcLoginAttemptId, OidcNonce, OidcPkceCodeVerifier, OidcState};

#[derive(Clone, Debug)]
pub struct OidcLoginAttempt {
    id: OidcLoginAttemptId,
    state: OidcState,
    nonce: OidcNonce,
    pkce_code_verifier: Option<OidcPkceCodeVerifier>,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    consumed_at: Option<DateTime<Utc>>,
}

impl OidcLoginAttempt {
    pub fn new(
        state: OidcState,
        nonce: OidcNonce,
        pkce_code_verifier: Option<OidcPkceCodeVerifier>,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: OidcLoginAttemptId::new(),
            state,
            nonce,
            pkce_code_verifier,
            created_at,
            expires_at,
            consumed_at: None,
        }
    }

    pub fn from_persisted(
        id: OidcLoginAttemptId,
        state: OidcState,
        nonce: OidcNonce,
        pkce_code_verifier: Option<OidcPkceCodeVerifier>,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        consumed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            state,
            nonce,
            pkce_code_verifier,
            created_at,
            expires_at,
            consumed_at,
        }
    }

    pub fn id(&self) -> &OidcLoginAttemptId {
        &self.id
    }

    pub fn state(&self) -> &OidcState {
        &self.state
    }

    pub fn nonce(&self) -> &OidcNonce {
        &self.nonce
    }

    pub fn pkce_code_verifier(&self) -> Option<&OidcPkceCodeVerifier> {
        self.pkce_code_verifier.as_ref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn consumed_at(&self) -> Option<DateTime<Utc>> {
        self.consumed_at
    }
}
