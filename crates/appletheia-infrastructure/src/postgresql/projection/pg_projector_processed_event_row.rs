use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgProjectorProcessedEventRow {
    pub id: Uuid,
    pub projector_name: String,
    pub event_id: Uuid,
    pub processed_at: DateTime<Utc>,
}
