use super::{
    AuthTokenExchangeCodeConsumedAt, AuthTokenExchangeCodeCreatedAt,
    AuthTokenExchangeCodeExpiresAt, AuthTokenExchangeCodeHash, AuthTokenExchangeCodeRecordId,
    EncryptedAuthTokenExchangeGrant, PkceCodeChallenge, PkceCodeChallengeMethod,
};

/// Represents persisted values used to reconstruct an exchange code record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenExchangeCodeRecordPersisted {
    pub id: AuthTokenExchangeCodeRecordId,
    pub code_hash: AuthTokenExchangeCodeHash,
    pub code_challenge_method: Option<PkceCodeChallengeMethod>,
    pub code_challenge: Option<PkceCodeChallenge>,
    pub encrypted_grant: EncryptedAuthTokenExchangeGrant,
    pub created_at: AuthTokenExchangeCodeCreatedAt,
    pub expires_at: AuthTokenExchangeCodeExpiresAt,
    pub consumed_at: Option<AuthTokenExchangeCodeConsumedAt>,
}
