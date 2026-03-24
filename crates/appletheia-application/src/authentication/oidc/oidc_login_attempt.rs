use super::{
    OidcLoginAttemptConsumedAt, OidcLoginAttemptCreatedAt, OidcLoginAttemptExpiresAt,
    OidcLoginAttemptId, OidcNonce, OidcState, PkceCodeVerifier,
};

#[derive(Clone, Debug)]
pub struct OidcLoginAttempt {
    id: OidcLoginAttemptId,
    state: OidcState,
    nonce: OidcNonce,
    pkce_code_verifier: Option<PkceCodeVerifier>,
    created_at: OidcLoginAttemptCreatedAt,
    expires_at: OidcLoginAttemptExpiresAt,
    consumed_at: Option<OidcLoginAttemptConsumedAt>,
}

impl OidcLoginAttempt {
    pub fn new(
        state: OidcState,
        nonce: OidcNonce,
        pkce_code_verifier: Option<PkceCodeVerifier>,
        created_at: OidcLoginAttemptCreatedAt,
        expires_at: OidcLoginAttemptExpiresAt,
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
        pkce_code_verifier: Option<PkceCodeVerifier>,
        created_at: OidcLoginAttemptCreatedAt,
        expires_at: OidcLoginAttemptExpiresAt,
        consumed_at: Option<OidcLoginAttemptConsumedAt>,
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

    pub fn pkce_code_verifier(&self) -> Option<&PkceCodeVerifier> {
        self.pkce_code_verifier.as_ref()
    }

    pub fn created_at(&self) -> OidcLoginAttemptCreatedAt {
        self.created_at
    }

    pub fn expires_at(&self) -> OidcLoginAttemptExpiresAt {
        self.expires_at
    }

    pub fn consumed_at(&self) -> Option<OidcLoginAttemptConsumedAt> {
        self.consumed_at
    }
}
