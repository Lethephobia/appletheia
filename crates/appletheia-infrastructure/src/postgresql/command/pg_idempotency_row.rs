use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct IdempotencyRow {
    pub id: Uuid,
    pub command_name: String,
    pub command_hash: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}
