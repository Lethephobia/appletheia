use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct PgReadModelWatchRow {
    pub watch_id: Uuid,
    pub endpoint_id: Uuid,
    pub generation: i64,
}
