pub(crate) struct PgSnapshotModel {
    pub(crate) id: Uuid,
    pub(crate) aggregate_type: String,
    pub(crate) aggregate_id: Uuid,
    pub(crate) aggregate_version: i64,
    pub(crate) state: serde_json::Value,
    pub(crate) created_at: DateTime<Utc>,
}
