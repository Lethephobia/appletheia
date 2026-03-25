use appletheia_application::authentication::oidc::{
    OidcLoginAttempt, OidcLoginAttemptConsumedAt, OidcLoginAttemptExpiresAt, OidcLoginAttemptId,
    OidcLoginAttemptStartedAt, OidcNonce, OidcState, PkceCodeVerifier,
};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::pg_oidc_login_attempt_row_error::PgOidcLoginAttemptRowError;

#[derive(Clone, Debug, FromRow)]
pub struct PgOidcLoginAttemptRow {
    pub id: Uuid,
    pub state: String,
    pub nonce: String,
    pub pkce_code_verifier: Option<String>,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub consumed_at: Option<DateTime<Utc>>,
}

impl PgOidcLoginAttemptRow {
    pub fn try_into_oidc_login_attempt(
        self,
    ) -> Result<OidcLoginAttempt, PgOidcLoginAttemptRowError> {
        let pkce_code_verifier = match self.pkce_code_verifier {
            None => None,
            Some(value) => Some(
                PkceCodeVerifier::try_from(value)
                    .map_err(|_| PgOidcLoginAttemptRowError::InvalidPkceCodeVerifier)?,
            ),
        };

        let state = OidcState::try_from(self.state)
            .map_err(|_| PgOidcLoginAttemptRowError::InvalidOidcState)?;
        let nonce = OidcNonce::try_from(self.nonce)
            .map_err(|_| PgOidcLoginAttemptRowError::InvalidOidcNonce)?;

        Ok(OidcLoginAttempt::from_persisted(
            OidcLoginAttemptId::from_uuid(self.id),
            state,
            nonce,
            pkce_code_verifier,
            OidcLoginAttemptStartedAt::from(self.started_at),
            OidcLoginAttemptExpiresAt::from(self.expires_at),
            self.consumed_at.map(OidcLoginAttemptConsumedAt::from),
        ))
    }
}
