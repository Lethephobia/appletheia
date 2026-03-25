use appletheia_application::authentication::oidc::{
    OidcContinuation, OidcContinuationConsumedAt, OidcContinuationExpiresAt, OidcState,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use super::pg_oidc_continuation_row_error::PgOidcContinuationRowError;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct PgOidcContinuationRow {
    pub id: Uuid,
    pub state: String,
    pub payload: serde_json::Value,
    pub expires_at: DateTime<Utc>,
    pub consumed_at: Option<DateTime<Utc>>,
}

impl PgOidcContinuationRow {
    pub fn try_into_oidc_continuation<P>(
        self,
    ) -> Result<OidcContinuation<P>, PgOidcContinuationRowError>
    where
        P: Serialize + DeserializeOwned,
    {
        let payload = serde_json::from_value(self.payload)
            .map_err(|_| PgOidcContinuationRowError::InvalidPayload)?;
        let state = OidcState::try_from(self.state)
            .map_err(|_| PgOidcContinuationRowError::InvalidOidcState)?;

        Ok(OidcContinuation::from_persisted(
            state,
            payload,
            OidcContinuationExpiresAt::from(self.expires_at),
            self.consumed_at.map(OidcContinuationConsumedAt::from),
        ))
    }
}
