pub mod aggregate_apply;
pub mod aggregate_error;
pub mod aggregate_id;
pub mod aggregate_state;
pub mod aggregate_state_access;
pub mod aggregate_type;
pub mod aggregate_uncommitted_events;
pub mod aggregate_version;
pub mod aggregate_version_access;
pub mod aggregate_version_error;
pub mod aggregate_version_range;
pub mod entity_id;

pub use aggregate_apply::AggregateApply;
pub use aggregate_error::AggregateError;
pub use aggregate_id::AggregateId;
pub use aggregate_state::AggregateState;
pub use aggregate_state_access::AggregateStateAccess;
pub use aggregate_type::AggregateType;
pub use aggregate_uncommitted_events::AggregateUncommittedEvents;
pub use aggregate_version::AggregateVersion;
pub use aggregate_version_access::AggregateVersionAccess;
pub use aggregate_version_error::AggregateVersionError;
pub use aggregate_version_range::AggregateVersionRange;
pub use entity_id::EntityId;

use std::{error::Error, fmt::Debug};

use crate::event::{Event, EventPayload};
use crate::snapshot::Snapshot;

pub trait Aggregate:
    Clone
    + Debug
    + Default
    + Send
    + Sync
    + 'static
    + AggregateStateAccess<Self::Id, Self::State>
    + AggregateVersionAccess
    + AggregateUncommittedEvents<Self::Id, Self::EventPayload>
    + AggregateApply<Self::EventPayload, Self::Error>
{
    type Id: AggregateId;
    type State: AggregateState<Id = Self::Id>;
    type EventPayload: EventPayload;
    type Error: Error + From<AggregateError<Self::Id>> + Send + Sync + 'static;

    const AGGREGATE_TYPE: AggregateType;

    fn aggregate_id(&self) -> Option<Self::Id> {
        self.state().map(|state| state.id())
    }

    fn bump_version(&mut self) -> Result<(), Self::Error> {
        let next_version = self.version().try_next().map_err(AggregateError::Version)?;
        self.set_version(next_version);
        Ok(())
    }

    fn append_event(&mut self, payload: Self::EventPayload) -> Result<(), Self::Error> {
        self.apply(&payload)?;
        self.bump_version()?;
        let aggregate_id = self
            .state()
            .as_ref()
            .map(|state| state.id())
            .ok_or(AggregateError::NoState)?;
        let event = Event::new(aggregate_id, self.version(), payload);
        self.record_uncommitted_event(event);
        Ok(())
    }

    fn validate_next_event(
        &self,
        event: &Event<Self::Id, Self::EventPayload>,
    ) -> Result<(), Self::Error> {
        if let Some(state) = self.state()
            && state.id() != event.aggregate_id()
        {
            return Err(
                AggregateError::InvalidAggregateId(state.id(), event.aggregate_id()).into(),
            );
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

    fn replay_event(
        &mut self,
        event: Event<Self::Id, Self::EventPayload>,
    ) -> Result<(), Self::Error> {
        self.validate_next_event(&event)?;
        self.apply(event.payload())?;
        self.bump_version()?;
        Ok(())
    }

    fn restore_snapshot(&mut self, snapshot: Snapshot<Self::State>) -> Result<(), Self::Error> {
        let version = snapshot.aggregate_version();
        let state = snapshot.into_state();
        self.set_state(Some(state));
        self.set_version(version);
        Ok(())
    }

    fn replay_events<I: IntoIterator<Item = Event<Self::Id, Self::EventPayload>>>(
        &mut self,
        events: I,
        snapshot: Option<Snapshot<Self::State>>,
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

    fn to_snapshot(&self) -> Result<Snapshot<Self::State>, Self::Error> {
        self.state()
            .map(|state| Snapshot::new(state.id(), self.version(), state.clone()))
            .ok_or(AggregateError::NoState.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fmt, fmt::Display};

    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::{Uuid, Version};

    use crate::aggregate::{AggregateError, AggregateId, AggregateVersion};

    #[derive(Debug, Error)]
    enum CounterIdError {
        #[error("not a uuidv7: {0}")]
        NotUuidV7(Uuid),
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
    #[serde(try_from = "Uuid", into = "Uuid")]
    struct CounterId(Uuid);

    impl CounterId {
        fn new() -> Self {
            Self(Uuid::now_v7())
        }
    }

    impl AggregateId for CounterId {
        type Error = CounterIdError;

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            match value.get_version() {
                Some(Version::SortRand) => Ok(Self(value)),
                _ => Err(CounterIdError::NotUuidV7(value)),
            }
        }

        fn value(&self) -> Uuid {
            self.0
        }
    }

    impl Display for CounterId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl From<CounterId> for Uuid {
        fn from(value: CounterId) -> Self {
            value.value()
        }
    }

    impl TryFrom<Uuid> for CounterId {
        type Error = CounterIdError;

        fn try_from(value: Uuid) -> Result<Self, Self::Error> {
            Self::try_from_uuid(value)
        }
    }

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error("json error: {0}")]
        Json(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
        counter: i32,
    }

    impl CounterState {
        fn new(id: CounterId, counter: i32) -> Self {
            Self { id, counter }
        }

        fn counter(&self) -> i32 {
            self.counter
        }
    }

    impl AggregateState for CounterState {
        type Id = CounterId;
        type Error = CounterStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error("json error: {0}")]
        Json(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum CounterEventPayload {
        Created(),
        Increment(i32),
        Decrement(i32),
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;
    }

    impl Display for CounterEventPayload {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    type CounterEvent = Event<CounterId, CounterEventPayload>;

    #[derive(Debug, Error)]
    enum CounterError {
        #[error("aggregate error: {0}")]
        Aggregate(#[from] AggregateError<CounterId>),

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

    #[derive(Clone, Debug)]
    struct Counter {
        id: CounterId,
        state: Option<CounterState>,
        version: AggregateVersion,
        uncommitted_events: Vec<CounterEvent>,
    }

    impl Counter {
        pub fn new() -> Self {
            let id = CounterId::new();
            Self {
                id,
                state: None,
                version: AggregateVersion::new(),
                uncommitted_events: Vec::new(),
            }
        }

        pub fn create(&mut self) -> Result<(), CounterError> {
            self.append_event(CounterEventPayload::Created())?;
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

    impl Default for Counter {
        fn default() -> Self {
            Self::new()
        }
    }

    impl AggregateStateAccess<CounterId, CounterState> for Counter {
        fn state(&self) -> Option<&CounterState> {
            self.state.as_ref()
        }

        fn set_state(&mut self, state: Option<CounterState>) {
            self.state = state;
        }
    }

    impl AggregateVersionAccess for Counter {
        fn version(&self) -> AggregateVersion {
            self.version
        }

        fn set_version(&mut self, version: AggregateVersion) {
            self.version = version;
        }
    }

    impl AggregateUncommittedEvents<CounterId, CounterEventPayload> for Counter {
        fn uncommitted_events(&self) -> &[Event<CounterId, CounterEventPayload>] {
            &self.uncommitted_events
        }

        fn record_uncommitted_event(&mut self, event: Event<CounterId, CounterEventPayload>) {
            self.uncommitted_events.push(event);
        }

        fn clear_uncommitted_events(&mut self) {
            self.uncommitted_events.clear();
        }
    }

    impl AggregateApply<CounterEventPayload, CounterError> for Counter {
        fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
            match payload {
                CounterEventPayload::Created() => {
                    if self.state.is_some() {
                        return Err(CounterError::InvalidEventPayload(payload.clone()).into());
                    }
                    self.state = Some(CounterState::new(self.id, 0));
                }
                CounterEventPayload::Increment(delta) => {
                    if self.state.is_none() {
                        return Err(CounterError::StateMissing.into());
                    }
                    self.state.as_mut().unwrap().counter += delta;
                }
                CounterEventPayload::Decrement(delta) => {
                    if self.state.is_none() {
                        return Err(CounterError::StateMissing.into());
                    }
                    self.state.as_mut().unwrap().counter -= delta;
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

        const AGGREGATE_TYPE: AggregateType = AggregateType::new("counter");
    }

    #[test]
    fn aggregate_id_is_none_before_creation_and_some_after() {
        let mut counter = Counter::new();

        assert!(counter.aggregate_id().is_none());

        counter.create().expect("create should succeed");

        let aggregate_id = counter
            .aggregate_id()
            .expect("id should exist after create");
        assert_eq!(
            aggregate_id,
            counter.state().expect("state should exist").id()
        );
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
        assert_eq!(event.aggregate_id(), state.id());
        assert_eq!(event.aggregate_version().value(), 1);
        assert_eq!(event.payload(), &CounterEventPayload::Created());
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
        assert_eq!(events[0].payload(), &CounterEventPayload::Created());
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
        counter.set_version(max_version);

        let err = counter
            .bump_version()
            .expect_err("overflow when bumping version should error");

        assert!(matches!(
            err,
            CounterError::Aggregate(AggregateError::Version(_))
        ));
        assert_eq!(counter.version(), max_version);
    }

    #[test]
    fn validate_next_event_returns_error_for_mismatched_id() {
        let mut counter = Counter::new();
        counter.create().expect("create should succeed");
        let mismatched_event = CounterEvent::new(
            CounterId::new(),
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
            counter.aggregate_id().expect("id should exist"),
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
            counter.aggregate_id().expect("id should exist"),
            next_version,
            CounterEventPayload::Increment(1),
        );

        counter
            .validate_next_event(&event)
            .expect("validation should pass");
    }

    #[test]
    fn replay_event_applies_payload_and_updates_version() {
        let mut counter = Counter::new();
        let event = CounterEvent::new(
            counter.id,
            counter.version().try_next().unwrap(),
            CounterEventPayload::Created(),
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
        let mut counter = Counter::new();
        let event = CounterEvent::new(
            counter.id,
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
        let mut counter = Counter::new();
        let snapshot_state = CounterState::new(counter.id, 10);
        let snapshot_version = AggregateVersion::try_from(3).unwrap();
        let snapshot = Snapshot::new(
            snapshot_state.id(),
            snapshot_version,
            snapshot_state.clone(),
        );
        let event1_version = snapshot_version.try_next().unwrap();
        let event2_version = event1_version.try_next().unwrap();
        let events = vec![
            CounterEvent::new(
                snapshot_state.id(),
                event1_version,
                CounterEventPayload::Increment(5),
            ),
            CounterEvent::new(
                snapshot_state.id(),
                event2_version,
                CounterEventPayload::Decrement(3),
            ),
        ];

        counter
            .replay_events(events.into_iter(), Some(snapshot))
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
            counter.aggregate_id().expect("id should exist"),
            event_version,
            CounterEventPayload::Increment(4),
        )];

        counter
            .replay_events(events.into_iter(), None)
            .expect("replay_events should succeed");

        let state = counter.state().expect("state should exist");
        assert_eq!(state.counter(), 4);
        assert_eq!(counter.version(), event_version);
    }

    #[test]
    fn restore_snapshot_sets_state_and_version() {
        let mut counter = Counter::new();
        let snapshot_state = CounterState::new(counter.id, 7);
        let snapshot_version = AggregateVersion::try_from(2).unwrap();
        let snapshot = Snapshot::new(
            snapshot_state.id(),
            snapshot_version,
            snapshot_state.clone(),
        );

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

        assert_eq!(
            snapshot.aggregate_id(),
            counter.aggregate_id().expect("id should exist")
        );
        assert_eq!(snapshot.aggregate_version(), counter.version());
        assert_eq!(
            snapshot.state(),
            counter.state().expect("state should exist")
        );
    }
}
