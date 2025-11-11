use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_domain::{
    Aggregate, AggregateId, AggregateState, AggregateVersion, MaterializedAt, Snapshot, SnapshotId,
};

use super::pg_snapshot_error::PgSnapshotError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub(crate) struct PgSnapshotRow {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_version: i64,
    pub state: serde_json::Value,
    pub materialized_at: DateTime<Utc>,
}

impl PgSnapshotRow {
    pub fn try_into_snapshot<A: Aggregate>(self) -> Result<Snapshot<A::State>, PgSnapshotError<A>> {
        let id = SnapshotId::try_from(self.id).map_err(PgSnapshotError::SnapshotId)?;
        let aggregate_id =
            A::Id::try_from_uuid(self.aggregate_id).map_err(PgSnapshotError::AggregateId)?;
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)
            .map_err(PgSnapshotError::AggregateVersion)?;
        let state =
            A::State::try_from_json_value(self.state).map_err(PgSnapshotError::AggregateState)?;
        Ok(Snapshot::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            state,
            MaterializedAt::from(self.materialized_at),
        ))
    }
}
