pub mod aggregate_apply;
pub mod aggregate_core;
pub mod aggregate_error;
pub mod aggregate_id;
pub mod aggregate_state;
pub mod aggregate_state_error;
pub mod aggregate_type;
pub mod aggregate_version;
pub mod aggregate_version_error;
pub mod aggregate_version_range;
pub mod reference_entries;
pub mod reference_indexes;
pub mod reference_key;
pub mod reference_values;
pub mod reference_values_error;
pub mod unique_constraints;
pub mod unique_entries;
pub mod unique_key;
pub mod unique_value;
pub mod unique_value_error;
pub mod unique_value_part;
pub mod unique_value_part_error;
pub mod unique_values;
pub mod unique_values_error;

pub use aggregate_apply::AggregateApply;
pub use aggregate_core::AggregateCore;
pub use aggregate_error::AggregateError;
pub use aggregate_id::AggregateId;
pub use aggregate_state::AggregateState;
pub use aggregate_state_error::AggregateStateError;
pub use aggregate_type::AggregateType;
pub use aggregate_version::AggregateVersion;
pub use aggregate_version_error::AggregateVersionError;
pub use aggregate_version_range::AggregateVersionRange;
pub use reference_entries::ReferenceEntries;
pub use reference_indexes::ReferenceIndexes;
pub use reference_key::ReferenceKey;
pub use reference_values::ReferenceValues;
pub use reference_values_error::ReferenceValuesError;
pub use unique_constraints::UniqueConstraints;
pub use unique_entries::UniqueEntries;
pub use unique_key::UniqueKey;
pub use unique_value::UniqueValue;
pub use unique_value_error::UniqueValueError;
pub use unique_value_part::UniqueValuePart;
pub use unique_value_part_error::UniqueValuePartError;
pub use unique_values::UniqueValues;
pub use unique_values_error::UniqueValuesError;

use std::{error::Error, fmt::Debug};

use crate::event::{Event, EventPayload};
use crate::snapshot::Snapshot;

/// Represents an event-sourced aggregate root.
///
/// Implementations provide access to the shared `AggregateCore` and define how
/// event payloads are applied to evolve aggregate state.
pub trait Aggregate:
    Clone + Debug + Default + Send + Sync + 'static + AggregateApply<Self::EventPayload, Self::Error>
{
    type Id: AggregateId;
    type State: AggregateState;
    type EventPayload: EventPayload;
    type Error: Error
        + From<AggregateError<Self::Id>>
        + From<<Self::State as AggregateState>::Error>
        + Send
        + Sync
        + 'static;

    const TYPE: AggregateType;

    /// Creates a new, uninitialized aggregate with a fresh identifier.
    fn new() -> Self;

    /// Creates an uninitialized aggregate for an existing identifier.
    fn from_id(id: Self::Id) -> Self;

    /// Returns the shared aggregate core.
    fn core(&self) -> &AggregateCore<Self::Id, Self::State, Self::EventPayload>;

    /// Returns the shared aggregate core as a mutable reference.
    fn core_mut(&mut self) -> &mut AggregateCore<Self::Id, Self::State, Self::EventPayload>;

    /// Returns the current aggregate state, if it has been initialized.
    fn state(&self) -> Option<&Self::State> {
        self.core().state()
    }

    /// Returns the current aggregate state as a mutable reference, if it has been initialized.
    fn state_mut(&mut self) -> Option<&mut Self::State> {
        self.core_mut().state_mut()
    }

    /// Replaces the current aggregate state.
    fn set_state(&mut self, state: Option<Self::State>) {
        self.core_mut().set_state(state);
    }

    /// Returns the current aggregate state or a `NoState` error.
    fn state_required(&self) -> Result<&Self::State, Self::Error> {
        self.state().ok_or(AggregateError::NoState.into())
    }

    /// Returns the current aggregate state mutably or a `NoState` error.
    fn state_required_mut(&mut self) -> Result<&mut Self::State, Self::Error> {
        self.state_mut().ok_or(AggregateError::NoState.into())
    }

    /// Returns the current aggregate version.
    fn version(&self) -> AggregateVersion {
        self.core().version()
    }

    /// Returns the recorded uncommitted events.
    fn uncommitted_events(&self) -> &[Event<Self::Id, Self::EventPayload>] {
        self.core().uncommitted_events()
    }

    /// Returns the aggregate identifier generated when the aggregate was constructed.
    fn aggregate_id(&self) -> Self::Id {
        self.core().aggregate_id()
    }

    /// Returns the unique-key entries derived from the aggregate state and identifier.
    ///
    /// Returns an empty set when the aggregate has not materialized state yet.
    fn unique_entries(&self) -> Result<UniqueEntries, Self::Error> {
        let Some(state) = self.state() else {
            return Ok(UniqueEntries::new());
        };

        Ok(state.unique_entries(self.aggregate_id().value())?)
    }

    /// Returns the reference-index entries derived from the aggregate state and identifier.
    ///
    /// Returns an empty set when the aggregate has not materialized state yet.
    fn reference_entries(&self) -> Result<ReferenceEntries, Self::Error> {
        let Some(state) = self.state() else {
            return Ok(ReferenceEntries::new());
        };

        Ok(state.reference_entries(self.aggregate_id().value())?)
    }

    /// Applies a new payload, bumps the aggregate version, and records the resulting event.
    fn append_event(&mut self, payload: Self::EventPayload) -> Result<(), Self::Error> {
        self.apply(&payload)?;
        self.core_mut()
            .bump_version()
            .map_err(AggregateError::Version)?;
        let aggregate_id = self.aggregate_id();
        let event = Event::new(aggregate_id, self.version(), payload);
        self.core_mut().record_uncommitted_event(event);
        Ok(())
    }

    /// Validates that an event can be applied as the next event in sequence.
    fn validate_next_event(
        &self,
        event: &Event<Self::Id, Self::EventPayload>,
    ) -> Result<(), Self::Error> {
        if self.aggregate_id() != event.aggregate_id() {
            return Err(AggregateError::InvalidAggregateId(
                self.aggregate_id(),
                event.aggregate_id(),
            )
            .into());
        }
        let next_version = self.version().try_next().map_err(AggregateError::Version)?;
        if event.aggregate_version() != next_version {
            return Err(AggregateError::InvalidNextEventVersion(
                event.aggregate_version(),
                next_version,
            )
            .into());
        }
        Ok(())
    }

    /// Replays a persisted event onto the aggregate.
    fn replay_event(
        &mut self,
        event: Event<Self::Id, Self::EventPayload>,
    ) -> Result<(), Self::Error> {
        self.validate_next_event(&event)?;
        self.apply(event.payload())?;
        self.core_mut()
            .bump_version()
            .map_err(AggregateError::Version)?;
        Ok(())
    }

    /// Restores aggregate state and version from a snapshot.
    fn restore_snapshot(
        &mut self,
        snapshot: Snapshot<Self::Id, Self::State>,
    ) -> Result<(), Self::Error> {
        if snapshot.aggregate_id() != self.aggregate_id() {
            return Err(AggregateError::InvalidAggregateId(
                self.aggregate_id(),
                snapshot.aggregate_id(),
            )
            .into());
        }
        let version = snapshot.aggregate_version();
        let state = snapshot.into_state();
        self.set_state(Some(state));
        self.core_mut().set_version(version);
        Ok(())
    }

    /// Restores an optional snapshot and replays the provided events in order.
    fn replay_events<I: IntoIterator<Item = Event<Self::Id, Self::EventPayload>>>(
        &mut self,
        events: I,
        snapshot: Option<Snapshot<Self::Id, Self::State>>,
    ) -> Result<(), Self::Error> {
        let mut event_iter = events.into_iter();

        let first_event = event_iter.next();
        if first_event.is_none() && snapshot.is_none() {
            return Err(AggregateError::NoEvents.into());
        }

        if let Some(snapshot) = snapshot {
            self.restore_snapshot(snapshot)?;
        }
        if let Some(first_event) = first_event {
            self.replay_event(first_event)?;
        }
        for event in event_iter {
            self.replay_event(event)?;
        }
        Ok(())
    }

    /// Materializes the current aggregate state into a snapshot.
    fn to_snapshot(&self) -> Result<Snapshot<Self::Id, Self::State>, Self::Error> {
        self.state()
            .map(|state| Snapshot::new(self.aggregate_id(), self.version(), state.clone()))
            .ok_or(AggregateError::NoState.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    use std::{fmt, fmt::Display};
    use thiserror::Error;
    use uuid::Uuid;

    use crate::aggregate::{
        AggregateError, AggregateId, AggregateState, AggregateStateError, AggregateVersion,
        ReferenceIndexes, UniqueConstraints,
    };
    use crate::event::{EventName, EventPayload};

    #[derive(Debug, Error)]
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

        fn new() -> Self {
            Self(Uuid::now_v7())
        }

        fn value(&self) -> Uuid {
            self.0
        }

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            validate_counter_id(value)?;
            Ok(Self(value))
        }
    }

    impl Display for CounterId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        counter: i32,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {}
    impl ReferenceIndexes<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Error = CounterStateError;
    }

    impl CounterState {
        fn new(counter: i32) -> Self {
            Self { counter }
        }

        fn counter(&self) -> i32 {
            self.counter
        }
    }

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Json(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum CounterEventPayload {
        Created,
        CreationRejected,
        Increment(i32),
        Decrement(i32),
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Created => EventName::new("created"),
                Self::CreationRejected => EventName::new("creation_rejected"),
                Self::Increment(..) => EventName::new("increment"),
                Self::Decrement(..) => EventName::new("decrement"),
            }
        }
    }

    impl Display for CounterEventPayload {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CounterEventPayload::Created => write!(f, "created"),
                CounterEventPayload::CreationRejected => write!(f, "creation_rejected"),
                CounterEventPayload::Increment(delta) => write!(f, "increment({delta})"),
                CounterEventPayload::Decrement(delta) => write!(f, "decrement({delta})"),
            }
        }
    }

    type CounterEvent = Event<CounterId, CounterEventPayload>;

    #[derive(Debug, Error)]
    enum CounterError {
        #[error("aggregate error: {0}")]
        Aggregate(#[from] AggregateError<CounterId>),

        #[error(transparent)]
        State(#[from] CounterStateError),

        #[error("invalid event payload: {0}")]
        InvalidEventPayload(CounterEventPayload),

        #[error("state missing")]
        StateMissing,
    }

    impl PartialEq for CounterError {
        fn eq(&self, other: &Self) -> bool {
            matches!(
                (self, other),
                (CounterError::StateMissing, CounterError::StateMissing)
                    | (CounterError::Aggregate(_), CounterError::Aggregate(_))
            )
        }
    }

    impl Eq for CounterError {}

    #[derive(Clone, Debug, Default)]
    struct Counter {
        core: AggregateCore<CounterId, CounterState, CounterEventPayload>,
    }

    impl Counter {
        pub fn create(&mut self) -> Result<(), CounterError> {
            self.append_event(CounterEventPayload::Created)?;
            Ok(())
        }

        pub fn reject_creation(&mut self) -> Result<(), CounterError> {
            self.append_event(CounterEventPayload::CreationRejected)?;
            Ok(())
        }

        pub fn increment(&mut self, delta: i32) -> Result<(), CounterError> {
            self.append_event(CounterEventPayload::Increment(delta))?;
            Ok(())
        }

        pub fn decrement(&mut self, delta: i32) -> Result<(), CounterError> {
            self.append_event(CounterEventPayload::Decrement(delta))?;
            Ok(())
        }
    }

    impl AggregateApply<CounterEventPayload, CounterError> for Counter {
        fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
            match payload {
                CounterEventPayload::Created => {
                    if self.state().is_some() {
                        return Err(CounterError::InvalidEventPayload(payload.clone()));
                    }
                    self.set_state(Some(CounterState::new(0)));
                }
                CounterEventPayload::CreationRejected => {}
                CounterEventPayload::Increment(delta) => {
                    let state = self.state_mut().ok_or(CounterError::StateMissing)?;
                    state.counter += delta;
                }
                CounterEventPayload::Decrement(delta) => {
                    let state = self.state_mut().ok_or(CounterError::StateMissing)?;
                    state.counter -= delta;
                }
            }
            Ok(())
        }
    }

    impl Aggregate for Counter {
        type Id = CounterId;
        type State = CounterState;
        type EventPayload = CounterEventPayload;
        type Error = CounterError;

        const TYPE: AggregateType = AggregateType::new("counter");

        fn new() -> Self {
            Self {
                core: AggregateCore::new(),
            }
        }

        fn from_id(id: Self::Id) -> Self {
            Self {
                core: AggregateCore::from_id(id),
            }
        }

        fn core(&self) -> &AggregateCore<Self::Id, Self::State, Self::EventPayload> {
            &self.core
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::Id, Self::State, Self::EventPayload> {
            &mut self.core
        }
    }

    #[test]
    fn aggregate_id_exists_before_creation() {
        let counter = Counter::new();

        assert!(!counter.aggregate_id().value().is_nil());
    }

    #[test]
    fn create_initializes_state_and_records_created_event() {
        let mut counter = Counter::new();

        counter.create().expect("create should succeed");

        let state = counter.state().expect("state should be initialized");
        assert_eq!(state.counter(), 0);
        assert_eq!(counter.version().value(), 1);
        let events = counter.uncommitted_events();
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.aggregate_id(), counter.aggregate_id());
        assert_eq!(event.aggregate_version().value(), 1);
        assert_eq!(event.payload(), &CounterEventPayload::Created);
    }

    #[test]
    fn creation_rejection_records_event_without_initializing_state() {
        let mut counter = Counter::new();

        counter
            .reject_creation()
            .expect("creation rejection should succeed");

        assert!(counter.state().is_none());
        assert!(
            counter
                .unique_entries()
                .expect("entries should build")
                .is_empty()
        );
        assert!(
            counter
                .reference_entries()
                .expect("entries should build")
                .is_empty()
        );
        assert_eq!(counter.version().value(), 1);
        assert_eq!(counter.uncommitted_events().len(), 1);
        assert_eq!(
            counter.uncommitted_events()[0].payload(),
            &CounterEventPayload::CreationRejected
        );
    }

    #[test]
    fn replay_creation_rejection_keeps_state_uninitialized() {
        let aggregate_id = CounterId::new();
        let event = Event::new(
            aggregate_id,
            AggregateVersion::try_from(1).expect("version should be valid"),
            CounterEventPayload::CreationRejected,
        );
        let mut counter = Counter::from_id(aggregate_id);

        counter
            .replay_events([event], None)
            .expect("creation rejection should replay");

        assert!(counter.state().is_none());
        assert_eq!(counter.version().value(), 1);
        assert!(counter.uncommitted_events().is_empty());
    }

    #[test]
    fn increment_and_decrement_update_state_and_version() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        counter.increment(5).expect("increment should succeed");
        counter.decrement(2).expect("decrement should succeed");

        let state = counter.state().expect("state should exist");
        assert_eq!(state.counter(), 3);
        assert_eq!(counter.version().value(), 3);

        let events = counter.uncommitted_events();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].payload(), &CounterEventPayload::Created);
        assert_eq!(events[1].payload(), &CounterEventPayload::Increment(5));
        assert_eq!(events[2].payload(), &CounterEventPayload::Decrement(2));
    }

    #[test]
    fn append_event_returns_error_when_state_missing() {
        let mut counter = Counter::new();

        let err = counter
            .increment(1)
            .expect_err("increment should fail without initial state");

        assert!(matches!(err, CounterError::StateMissing));
        assert_eq!(counter.version().value(), 0);
        assert!(counter.uncommitted_events().is_empty());
    }

    #[test]
    fn bump_version_returns_error_on_overflow() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        let max_version = AggregateVersion::try_from(i64::MAX).unwrap();
        counter.core_mut().set_version(max_version);

        let err = counter
            .core_mut()
            .bump_version()
            .expect_err("overflow when bumping version should error");

        assert!(matches!(err, AggregateVersionError::Overflow));
        assert_eq!(counter.version(), max_version);
    }

    #[test]
    fn validate_next_event_returns_error_for_mismatched_id() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        let mismatched_event = CounterEvent::new(
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted"),
            counter.version().try_next().unwrap(),
            CounterEventPayload::Increment(1),
        );

        let err = counter
            .validate_next_event(&mismatched_event)
            .expect_err("expected invalid aggregate id error");

        assert!(matches!(
            err,
            CounterError::Aggregate(AggregateError::InvalidAggregateId(_, _))
        ));
    }

    #[test]
    fn validate_next_event_returns_error_for_incorrect_version() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        let invalid_version_event = CounterEvent::new(
            counter.aggregate_id(),
            counter.version(),
            CounterEventPayload::Increment(1),
        );

        let err = counter
            .validate_next_event(&invalid_version_event)
            .expect_err("expected invalid version error");

        assert!(matches!(
            err,
            CounterError::Aggregate(AggregateError::InvalidNextEventVersion(_, _))
        ));
    }

    #[test]
    fn validate_next_event_succeeds_for_expected_sequence() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        let next_version = counter.version().try_next().unwrap();
        let event = CounterEvent::new(
            counter.aggregate_id(),
            next_version,
            CounterEventPayload::Increment(1),
        );

        counter
            .validate_next_event(&event)
            .expect("validation should pass");
    }

    #[test]
    fn replay_event_applies_payload_and_updates_version() {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut counter = Counter::from_id(id);
        let event = CounterEvent::new(
            id,
            counter.version().try_next().unwrap(),
            CounterEventPayload::Created,
        );

        counter
            .replay_event(event)
            .expect("replay_event should succeed");

        let state = counter.state().expect("state should exist after replay");
        assert_eq!(state.counter(), 0);
        assert_eq!(counter.version().value(), 1);
        assert!(counter.uncommitted_events().is_empty());
    }

    #[test]
    fn replay_event_propagates_apply_errors() {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut counter = Counter::from_id(id);
        let event = CounterEvent::new(
            id,
            counter.version().try_next().unwrap(),
            CounterEventPayload::Increment(1),
        );

        let err = counter
            .replay_event(event)
            .expect_err("expected apply error to propagate");

        assert!(matches!(err, CounterError::StateMissing));
        assert_eq!(counter.version().value(), 0);
    }

    #[test]
    fn replay_events_applies_snapshot_and_replays_sequence() {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut counter = Counter::from_id(id);
        let snapshot_state = CounterState::new(10);
        let snapshot_version = AggregateVersion::try_from(3).unwrap();
        let snapshot = Snapshot::new(id, snapshot_version, snapshot_state.clone());
        let event1_version = snapshot_version.try_next().unwrap();
        let event2_version = event1_version.try_next().unwrap();
        let events = vec![
            CounterEvent::new(id, event1_version, CounterEventPayload::Increment(5)),
            CounterEvent::new(id, event2_version, CounterEventPayload::Decrement(3)),
        ];

        counter
            .replay_events(events, Some(snapshot))
            .expect("replay_events should succeed");

        let state = counter.state().expect("state should exist after replay");
        assert_eq!(state.counter(), 12);
        assert_eq!(counter.version(), event2_version);
        assert!(counter.uncommitted_events().is_empty());
    }

    #[test]
    fn replay_events_without_snapshot_starts_from_current_state() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        let event_version = counter.version().try_next().unwrap();
        let events = vec![CounterEvent::new(
            counter.aggregate_id(),
            event_version,
            CounterEventPayload::Increment(4),
        )];

        counter
            .replay_events(events, None)
            .expect("replay_events should succeed");

        let state = counter.state().expect("state should exist");
        assert_eq!(state.counter(), 4);
        assert_eq!(counter.version(), event_version);
    }

    #[test]
    fn restore_snapshot_sets_state_and_version() {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut counter = Counter::from_id(id);
        let snapshot_state = CounterState::new(7);
        let snapshot_version = AggregateVersion::try_from(2).unwrap();
        let snapshot = Snapshot::new(id, snapshot_version, snapshot_state.clone());

        counter
            .restore_snapshot(snapshot)
            .expect("restore_snapshot should succeed");

        let state = counter.state().expect("state should exist after restore");
        assert_eq!(state, &snapshot_state);
        assert_eq!(counter.version(), snapshot_version);
    }

    #[test]
    fn to_snapshot_returns_error_when_state_missing() {
        let counter = Counter::new();

        let err = counter
            .to_snapshot()
            .expect_err("expected error when state missing");

        assert!(matches!(
            err,
            CounterError::Aggregate(AggregateError::NoState)
        ));
    }

    #[test]
    fn to_snapshot_serializes_current_state() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        counter.increment(3).expect("increment should succeed");

        let snapshot = counter
            .to_snapshot()
            .expect("expected snapshot to be created");

        assert_eq!(snapshot.aggregate_id(), counter.aggregate_id());
        assert_eq!(snapshot.aggregate_version(), counter.version());
        assert_eq!(
            snapshot.state(),
            counter.state().expect("state should exist")
        );
    }
}
