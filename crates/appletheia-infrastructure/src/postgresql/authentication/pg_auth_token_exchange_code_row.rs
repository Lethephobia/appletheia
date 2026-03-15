use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::authentication::{
    AuthTokenExchangeCodeConsumedAt, AuthTokenExchangeCodeCreatedAt,
    AuthTokenExchangeCodeExpiresAt, AuthTokenExchangeCodeHash, AuthTokenExchangeCodeRecord,
    AuthTokenExchangeCodeRecordId, AuthTokenExchangeCodeRecordPersisted,
    AuthTokenExchangeCodeStoreError, EncryptedAuthTokenExchangeGrant, PkceCodeChallenge,
    PkceCodeChallengeMethod,
};

/// Represents a persisted auth token exchange code record in PostgreSQL.
#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct PgAuthTokenExchangeCodeRow {
    pub id: Uuid,
    pub code_hash: String,
    pub code_challenge_method: Option<String>,
    pub code_challenge: Option<String>,
    pub encrypted_grant: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub consumed_at: Option<DateTime<Utc>>,
}

impl PgAuthTokenExchangeCodeRow {
    pub fn try_into_record(
        self,
    ) -> Result<AuthTokenExchangeCodeRecord, AuthTokenExchangeCodeStoreError> {
        let code_challenge_method = self
            .code_challenge_method
            .as_deref()
            .map(str::parse::<PkceCodeChallengeMethod>)
            .transpose()
            .map_err(|source| {
                AuthTokenExchangeCodeStoreError::Backend(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    source,
                )))
            })?;
        let code_challenge = self.code_challenge.map(PkceCodeChallenge::new);
        let code_hash = AuthTokenExchangeCodeHash::new(self.code_hash).map_err(|source| {
            AuthTokenExchangeCodeStoreError::Backend(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                source,
            )))
        })?;

        Ok(AuthTokenExchangeCodeRecord::from_persisted(
            AuthTokenExchangeCodeRecordPersisted {
                id: AuthTokenExchangeCodeRecordId::from(self.id),
                code_hash,
                code_challenge_method,
                code_challenge,
                encrypted_grant: EncryptedAuthTokenExchangeGrant::new(self.encrypted_grant),
                created_at: AuthTokenExchangeCodeCreatedAt::from(self.created_at),
                expires_at: AuthTokenExchangeCodeExpiresAt::from(self.expires_at),
                consumed_at: self.consumed_at.map(AuthTokenExchangeCodeConsumedAt::from),
            },
        ))
    }
}
