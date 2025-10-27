use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct PgEventModel {
    aggregate_type: String,
    aggregate_id: Uuid,
    aggregate_version: i64,
    event_type: String,
    payload: serde_json::Value,
    created_at: DateTime<Utc>,
}
