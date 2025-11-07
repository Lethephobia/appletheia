use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub(crate) struct PgEventModel {
    pub event_sequence: i64,
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_version: i64,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub recorded_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub context: serde_json::Value,
}
