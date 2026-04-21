pub mod snapshot_id;
pub mod snapshot_id_error;
pub mod snapshot_materialized_at;

pub use snapshot_id::SnapshotId;
pub use snapshot_id_error::SnapshotIdError;
pub use snapshot_materialized_at::SnapshotMaterializedAt;

use crate::aggregate::{AggregateState, AggregateVersion};

/// Represents a materialized snapshot of aggregate state at a specific version.
///
/// A snapshot captures the aggregate identifier, the version at which the state
/// was materialized, the serialized state itself, and snapshot metadata.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Snapshot<S: AggregateState> {
    id: SnapshotId,
    aggregate_id: S::Id,
    aggregate_version: AggregateVersion,
    state: S,
    materialized_at: SnapshotMaterializedAt,
}

impl<S: AggregateState> Snapshot<S> {
    /// Creates a new snapshot with a fresh snapshot ID and the current timestamp.
    pub fn new(aggregate_id: S::Id, aggregate_version: AggregateVersion, state: S) -> Self {
        Self {
            id: SnapshotId::new(),
            aggregate_id,
            aggregate_version,
            state,
            materialized_at: SnapshotMaterializedAt::now(),
        }
    }

    /// Rebuilds a snapshot from already persisted values.
    pub fn from_persisted(
        id: SnapshotId,
        aggregate_id: S::Id,
        aggregate_version: AggregateVersion,
        state: S,
        materialized_at: SnapshotMaterializedAt,
    ) -> Self {
        Self {
            id,
            aggregate_id,
            aggregate_version,
            state,
            materialized_at,
        }
    }

    /// Returns the snapshot identifier.
    pub fn id(&self) -> SnapshotId {
        self.id
    }

    /// Returns the identifier of the aggregate represented by the snapshot.
    pub fn aggregate_id(&self) -> S::Id {
        self.aggregate_id
    }

    /// Returns the aggregate version captured by the snapshot.
    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    /// Returns the captured aggregate state.
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Consumes the snapshot and returns the captured aggregate state.
    pub fn into_state(self) -> S {
        self.state
    }

    /// Returns the timestamp at which the snapshot was materialized.
    pub fn materialized_at(&self) -> SnapshotMaterializedAt {
        self.materialized_at
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::{Snapshot, SnapshotId, SnapshotMaterializedAt};
    use crate::aggregate::{
        AggregateId, AggregateState, AggregateStateError, AggregateVersion, UniqueConstraints,
    };

    #[derive(Debug, Error, Eq, PartialEq)]
    enum CounterIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
    }

    fn validate_counter_id(value: Uuid) -> Result<(), CounterIdError> {
        if value.is_nil() {
            return Err(CounterIdError::NilUuid);
        }

        Ok(())
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    struct CounterId(Uuid);

    impl AggregateId for CounterId {
        type Error = CounterIdError;

        fn value(&self) -> Uuid {
            self.0
        }

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            validate_counter_id(value)?;
            Ok(Self(value))
        }
    }

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
        count: i32,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Id = CounterId;
        type Error = CounterStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[test]
    fn new_creates_snapshot_with_generated_id_and_timestamp() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let aggregate_version = AggregateVersion::try_from(3).expect("version should be valid");
        let state = CounterState {
            id: aggregate_id,
            count: 2,
        };
        let before = Utc::now();

        let snapshot = Snapshot::new(aggregate_id, aggregate_version, state.clone());

        let after = Utc::now();
        assert_eq!(snapshot.aggregate_id(), aggregate_id);
        assert_eq!(snapshot.aggregate_version(), aggregate_version);
        assert_eq!(snapshot.state(), &state);
        assert!(snapshot.materialized_at().value() >= before);
        assert!(snapshot.materialized_at().value() <= after);
        let _ = snapshot.id();
    }

    #[test]
    fn from_persisted_preserves_all_fields() {
        let id = SnapshotId::try_from(Uuid::now_v7()).expect("uuidv7 should be accepted");
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let aggregate_version = AggregateVersion::try_from(7).expect("version should be valid");
        let state = CounterState {
            id: aggregate_id,
            count: 5,
        };
        let materialized_at = SnapshotMaterializedAt::from(Utc::now());

        let snapshot = Snapshot::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            state.clone(),
            materialized_at,
        );

        assert_eq!(snapshot.id(), id);
        assert_eq!(snapshot.aggregate_id(), aggregate_id);
        assert_eq!(snapshot.aggregate_version(), aggregate_version);
        assert_eq!(snapshot.state(), &state);
        assert_eq!(snapshot.materialized_at(), materialized_at);
    }

    #[test]
    fn into_state_returns_captured_state() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let state = CounterState {
            id: aggregate_id,
            count: 9,
        };
        let snapshot = Snapshot::new(
            aggregate_id,
            AggregateVersion::try_from(1).expect("version should be valid"),
            state.clone(),
        );

        let restored = snapshot.into_state();

        assert_eq!(restored, state);
    }
}
