use super::{
    AuthTokenExchangeCodeConsumedAt, AuthTokenExchangeCodeCreatedAt,
    AuthTokenExchangeCodeExpiresAt, AuthTokenExchangeCodeExpiresIn, AuthTokenExchangeCodeHash,
    AuthTokenExchangeCodeRecordId, AuthTokenExchangeCodeRecordPersisted,
    EncryptedAuthTokenExchangeGrant, PkceCodeChallenge, PkceCodeChallengeMethod,
};

/// Persists the encrypted exchange code grant and validation metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeCodeRecord {
    id: AuthTokenExchangeCodeRecordId,
    code_hash: AuthTokenExchangeCodeHash,
    code_challenge_method: Option<PkceCodeChallengeMethod>,
    code_challenge: Option<PkceCodeChallenge>,
    encrypted_grant: EncryptedAuthTokenExchangeGrant,
    created_at: AuthTokenExchangeCodeCreatedAt,
    expires_at: AuthTokenExchangeCodeExpiresAt,
    consumed_at: Option<AuthTokenExchangeCodeConsumedAt>,
}

impl AuthTokenExchangeCodeRecord {
    /// Creates a new exchange code record.
    pub fn new(
        code_hash: AuthTokenExchangeCodeHash,
        code_challenge_method: Option<PkceCodeChallengeMethod>,
        code_challenge: Option<PkceCodeChallenge>,
        encrypted_grant: EncryptedAuthTokenExchangeGrant,
        expires_in: AuthTokenExchangeCodeExpiresIn,
    ) -> Self {
        let created_at = AuthTokenExchangeCodeCreatedAt::now();

        Self {
            id: AuthTokenExchangeCodeRecordId::new(),
            code_hash,
            code_challenge_method,
            code_challenge,
            encrypted_grant,
            created_at,
            expires_at: AuthTokenExchangeCodeExpiresAt::from(
                created_at.value() + expires_in.value(),
            ),
            consumed_at: None,
        }
    }

    /// Reconstructs a persisted exchange code record.
    pub fn from_persisted(persisted: AuthTokenExchangeCodeRecordPersisted) -> Self {
        Self {
            id: persisted.id,
            code_hash: persisted.code_hash,
            code_challenge_method: persisted.code_challenge_method,
            code_challenge: persisted.code_challenge,
            encrypted_grant: persisted.encrypted_grant,
            created_at: persisted.created_at,
            expires_at: persisted.expires_at,
            consumed_at: persisted.consumed_at,
        }
    }

    /// Returns the record identifier.
    pub fn id(&self) -> AuthTokenExchangeCodeRecordId {
        self.id
    }

    /// Returns the hashed exchange code value.
    pub fn code_hash(&self) -> &AuthTokenExchangeCodeHash {
        &self.code_hash
    }

    /// Returns the configured challenge method, if present.
    pub fn code_challenge_method(&self) -> Option<PkceCodeChallengeMethod> {
        self.code_challenge_method
    }

    /// Returns the configured challenge, if present.
    pub fn code_challenge(&self) -> Option<&PkceCodeChallenge> {
        self.code_challenge.as_ref()
    }

    /// Returns the encrypted grant.
    pub fn encrypted_grant(&self) -> &EncryptedAuthTokenExchangeGrant {
        &self.encrypted_grant
    }

    /// Returns when the record was created.
    pub fn created_at(&self) -> AuthTokenExchangeCodeCreatedAt {
        self.created_at
    }

    /// Returns when the record expires.
    pub fn expires_at(&self) -> AuthTokenExchangeCodeExpiresAt {
        self.expires_at
    }

    /// Returns when the record was consumed, if it has been redeemed.
    pub fn consumed_at(&self) -> Option<AuthTokenExchangeCodeConsumedAt> {
        self.consumed_at
    }
}
