use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgSagaProcessedEventRow {
    pub id: Uuid,
    pub saga_name: String,
    pub correlation_id: Uuid,
    pub event_id: Uuid,
    pub processed_at: DateTime<Utc>,
}
