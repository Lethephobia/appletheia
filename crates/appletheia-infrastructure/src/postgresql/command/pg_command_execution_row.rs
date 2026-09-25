use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PgCommandExecutionRow {
    pub id: Uuid,
    pub message_id: Uuid,
    pub command_name: String,
    pub attempt_count: i64,
    pub lease_until: Option<DateTime<Utc>>,
    pub succeeded_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
}
