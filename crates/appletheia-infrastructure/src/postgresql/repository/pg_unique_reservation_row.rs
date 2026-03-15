use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgUniqueReservationRow {
    pub id: Uuid,
    pub aggregate_type: String,
    pub owner_aggregate_id: Uuid,
    pub namespace: String,
    pub normalized_value: String,
}
