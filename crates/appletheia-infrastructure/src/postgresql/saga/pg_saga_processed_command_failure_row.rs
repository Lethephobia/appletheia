use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgSagaProcessedCommandFailureRow {
    pub id: Uuid,
    pub saga_instance_id: Uuid,
    pub command_failure_id: Uuid,
    pub command_message_id: Uuid,
    pub processed_at: DateTime<Utc>,
}
