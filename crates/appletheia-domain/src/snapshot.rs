pub mod materialized_at;
pub mod snapshot_id;
pub mod snapshot_id_error;

pub use materialized_at::MaterializedAt;
pub use snapshot_id::SnapshotId;
pub use snapshot_id_error::SnapshotIdError;

use crate::aggregate::{AggregateState, AggregateVersion};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Snapshot<S: AggregateState> {
    id: SnapshotId,
    aggregate_id: S::Id,
    aggregate_version: AggregateVersion,
    state: S,
    materialized_at: MaterializedAt,
}

impl<S: AggregateState> Snapshot<S> {
    pub fn new(aggregate_id: S::Id, aggregate_version: AggregateVersion, state: S) -> Self {
        Self {
            id: SnapshotId::new(),
            aggregate_id,
            aggregate_version,
            state,
            materialized_at: MaterializedAt::now(),
        }
    }
    pub fn from_persisted(
        id: SnapshotId,
        aggregate_id: S::Id,
        aggregate_version: AggregateVersion,
        state: S,
        materialized_at: MaterializedAt,
    ) -> Self {
        Self {
            id,
            aggregate_id,
            aggregate_version,
            state,
            materialized_at,
        }
    }

    pub fn id(&self) -> SnapshotId {
        self.id
    }

    pub fn aggregate_id(&self) -> S::Id {
        self.state.id()
    }

    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn into_state(self) -> S {
        self.state
    }

    pub fn materialized_at(&self) -> MaterializedAt {
        self.materialized_at
    }
}
