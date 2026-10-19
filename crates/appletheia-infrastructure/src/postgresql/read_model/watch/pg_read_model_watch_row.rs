use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct PgReadModelWatchRow {
    pub watch_id: Uuid,
    pub endpoint_id: Uuid,
    pub lease_until: DateTime<Utc>,
    pub generation: i64,
}
