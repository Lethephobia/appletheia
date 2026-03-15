use appletheia_application::authentication::oidc::{
    OidcLoginAttempt, OidcLoginAttemptId, OidcNonce, OidcState, PkceCodeVerifier,
};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::pg_oidc_login_attempt_row_error::PgOidcLoginAttemptRowError;

#[derive(Clone, Debug, FromRow)]
pub struct PgOidcLoginAttemptRow {
    pub id: Uuid,
    pub state: Uuid,
    pub nonce: Uuid,
    pub pkce_code_verifier: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub consumed_at: Option<DateTime<Utc>>,
}

impl PgOidcLoginAttemptRow {
    pub fn try_into_oidc_login_attempt(
        mut self,
    ) -> Result<OidcLoginAttempt, PgOidcLoginAttemptRowError> {
        let consumed_at = self.consumed_at.take();
        self.try_into_oidc_login_attempt_with_consumed_at(consumed_at)
    }

    pub fn try_into_oidc_login_attempt_with_consumed_at(
        self,
        consumed_at: Option<DateTime<Utc>>,
    ) -> Result<OidcLoginAttempt, PgOidcLoginAttemptRowError> {
        let pkce_code_verifier = match self.pkce_code_verifier {
            None => None,
            Some(value) => Some(
                PkceCodeVerifier::try_from(value)
                    .map_err(|_| PgOidcLoginAttemptRowError::InvalidPkceCodeVerifier)?,
            ),
        };

        Ok(OidcLoginAttempt::from_persisted(
            OidcLoginAttemptId::from_uuid(self.id),
            OidcState::from(self.state),
            OidcNonce::from(self.nonce),
            pkce_code_verifier,
            self.created_at,
            self.expires_at,
            consumed_at,
        ))
    }
}
