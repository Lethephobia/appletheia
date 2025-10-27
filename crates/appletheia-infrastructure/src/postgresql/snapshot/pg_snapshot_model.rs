pub(crate) struct PgSnapshotModel {
    aggregate_type: String,
    aggregate_id: Uuid,
    aggregate_version: i64,
    state: serde_json::Value,
    created_at: DateTime<Utc>,
}
