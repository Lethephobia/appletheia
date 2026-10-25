use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct PgReadModelWatchEndpointRow {
    pub endpoint_id: Uuid,
    pub lease_until: DateTime<Utc>,
}
