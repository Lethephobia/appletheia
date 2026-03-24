use super::{
    OidcLoginAttemptConsumedAt, OidcLoginAttemptExpiresAt, OidcLoginAttemptId,
    OidcLoginAttemptStartedAt, OidcNonce, OidcState, PkceCodeVerifier,
};

#[derive(Clone, Debug)]
pub struct OidcLoginAttempt {
    id: OidcLoginAttemptId,
    state: OidcState,
    nonce: OidcNonce,
    pkce_code_verifier: Option<PkceCodeVerifier>,
    started_at: OidcLoginAttemptStartedAt,
    expires_at: OidcLoginAttemptExpiresAt,
    consumed_at: Option<OidcLoginAttemptConsumedAt>,
}

impl OidcLoginAttempt {
    pub fn new(
        state: OidcState,
        nonce: OidcNonce,
        pkce_code_verifier: Option<PkceCodeVerifier>,
        started_at: OidcLoginAttemptStartedAt,
        expires_at: OidcLoginAttemptExpiresAt,
    ) -> Self {
        Self {
            id: OidcLoginAttemptId::new(),
            state,
            nonce,
            pkce_code_verifier,
            started_at,
            expires_at,
            consumed_at: None,
        }
    }

    pub fn from_persisted(
        id: OidcLoginAttemptId,
        state: OidcState,
        nonce: OidcNonce,
        pkce_code_verifier: Option<PkceCodeVerifier>,
        started_at: OidcLoginAttemptStartedAt,
        expires_at: OidcLoginAttemptExpiresAt,
        consumed_at: Option<OidcLoginAttemptConsumedAt>,
    ) -> Self {
        Self {
            id,
            state,
            nonce,
            pkce_code_verifier,
            started_at,
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

    pub fn pkce_code_verifier(&self) -> Option<&PkceCodeVerifier> {
        self.pkce_code_verifier.as_ref()
    }

    pub fn started_at(&self) -> OidcLoginAttemptStartedAt {
        self.started_at
    }

    pub fn expires_at(&self) -> OidcLoginAttemptExpiresAt {
        self.expires_at
    }

    pub fn consumed_at(&self) -> Option<OidcLoginAttemptConsumedAt> {
        self.consumed_at
    }
}
