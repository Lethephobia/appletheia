use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgProjectionCheckpointRow {
    pub id: Uuid,
    pub projector_name: String,
    pub last_event_sequence: i64,
}
