use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub(super) struct IdempotencyRow {
    pub command_name: String,
    pub request_hash: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}
