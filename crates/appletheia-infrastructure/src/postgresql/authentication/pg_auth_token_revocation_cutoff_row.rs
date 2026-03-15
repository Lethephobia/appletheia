use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct PgAuthTokenRevocationCutoffRow {
    pub id: Uuid,
    pub subject_aggregate_type: String,
    pub subject_aggregate_id: Uuid,
    pub revoke_before: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
