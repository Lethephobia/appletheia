use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct PgAuthTokenRevocationRow {
    pub id: Uuid,
    pub token_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: DateTime<Utc>,
}
