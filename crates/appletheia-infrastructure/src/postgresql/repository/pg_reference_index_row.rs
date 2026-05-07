use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgReferenceIndexRow {
    pub id: Uuid,
    pub source_aggregate_type: String,
    pub source_aggregate_id: Uuid,
    pub namespace: String,
    pub target_aggregate_id: Uuid,
}
