use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub(crate) struct PgEventModel {
    pub(crate) id: Uuid,
    pub(crate) aggregate_type: String,
    pub(crate) aggregate_id: Uuid,
    pub(crate) aggregate_version: i64,
    pub(crate) payload: serde_json::Value,
    pub(crate) created_at: DateTime<Utc>,
}
