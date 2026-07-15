use crate::event::{Event, EventPayload};

use super::{AggregateId, AggregateState, AggregateVersion, AggregateVersionError};

/// Stores the mutable bookkeeping shared by aggregate implementations.
///
/// The core tracks the mandatory aggregate identifier, current state, latest
/// aggregate version, and uncommitted events produced since the last persistence boundary.
#[derive(Clone, Debug)]
pub struct AggregateCore<I, S, P>
where
    I: AggregateId,
    S: AggregateState,
    P: EventPayload,
{
    aggregate_id: I,
    state: Option<S>,
    version: AggregateVersion,
    uncommitted_events: Vec<Event<I, P>>,
}

impl<I, S, P> AggregateCore<I, S, P>
where
    I: AggregateId,
    S: AggregateState,
    P: EventPayload,
{
    /// Creates an aggregate core with a fresh ID, no state, version `0`, and no events.
    pub fn new() -> Self {
        Self::from_id(I::new())
    }

    /// Creates an empty aggregate core for an existing aggregate ID.
    pub fn from_id(aggregate_id: I) -> Self {
        Self {
            aggregate_id,
            state: None,
            version: AggregateVersion::new(),
            uncommitted_events: Vec::new(),
        }
    }

    /// Returns the aggregate identifier.
    pub fn aggregate_id(&self) -> I {
        self.aggregate_id
    }

    /// Returns the current aggregate state, if it has been initialized.
    pub fn state(&self) -> Option<&S> {
        self.state.as_ref()
    }

    /// Returns the current aggregate state as a mutable reference, if it has been initialized.
    pub fn state_mut(&mut self) -> Option<&mut S> {
        self.state.as_mut()
    }

    /// Replaces the current aggregate state.
    pub fn set_state(&mut self, state: Option<S>) {
        self.state = state;
    }

    /// Returns the current aggregate version.
    pub fn version(&self) -> AggregateVersion {
        self.version
    }

    /// Replaces the current aggregate version.
    pub fn set_version(&mut self, version: AggregateVersion) {
        self.version = version;
    }

    /// Advances the aggregate version by one.
    pub(crate) fn bump_version(&mut self) -> Result<(), AggregateVersionError> {
        let next_version = self.version.try_next()?;
        self.version = next_version;
        Ok(())
    }

    /// Returns the currently recorded uncommitted events.
    pub fn uncommitted_events(&self) -> &[Event<I, P>] {
        &self.uncommitted_events
    }

    /// Records an uncommitted event.
    pub fn record_uncommitted_event(&mut self, event: Event<I, P>) {
        self.uncommitted_events.push(event);
    }

    /// Removes all recorded uncommitted events.
    pub fn clear_uncommitted_events(&mut self) {
        self.uncommitted_events.clear();
    }
}

impl<I, S, P> Default for AggregateCore<I, S, P>
where
    I: AggregateId,
    S: AggregateState,
    P: EventPayload,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::AggregateCore;
    use crate::aggregate::AggregateState;
    use crate::aggregate::{
        AggregateId, AggregateStateError, AggregateVersion, AggregateVersionError,
        ReferenceIndexes, UniqueConstraints,
    };
    use crate::event::{Event, EventName, EventPayload};

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
    impl ReferenceIndexes<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Error = CounterStateError;
    }

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum CounterEventPayload {
        Incremented { amount: i32 },
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Incremented { .. } => EventName::new("incremented"),
            }
        }
    }

    #[test]
    fn new_initializes_empty_core() {
        let core = AggregateCore::<CounterId, CounterState, CounterEventPayload>::new();

        assert!(core.state().is_none());
        assert_eq!(core.version(), AggregateVersion::new());
        assert!(core.uncommitted_events().is_empty());
    }

    #[test]
    fn default_matches_new() {
        let core = AggregateCore::<CounterId, CounterState, CounterEventPayload>::default();

        assert!(core.state().is_none());
        assert_eq!(core.version(), AggregateVersion::new());
        assert!(core.uncommitted_events().is_empty());
    }

    #[test]
    fn from_id_initializes_core_with_provided_id() {
        let aggregate_id = CounterId::new();
        let core =
            AggregateCore::<CounterId, CounterState, CounterEventPayload>::from_id(aggregate_id);

        assert_eq!(core.aggregate_id(), aggregate_id);
        assert!(core.state().is_none());
        assert_eq!(core.version(), AggregateVersion::new());
        assert!(core.uncommitted_events().is_empty());
    }

    #[test]
    fn state_accessors_read_and_update_state() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut core = AggregateCore::<CounterId, CounterState, CounterEventPayload>::new();

        core.set_state(Some(CounterState {
            id: aggregate_id,
            count: 1,
        }));

        assert_eq!(core.state().expect("state should exist").count, 1);

        let state = core.state_mut().expect("state should exist");
        state.count += 2;

        assert_eq!(core.state().expect("state should exist").count, 3);
    }

    #[test]
    fn set_version_and_bump_version_update_version() {
        let mut core = AggregateCore::<CounterId, CounterState, CounterEventPayload>::new();
        let version = AggregateVersion::try_from(3).expect("version should be valid");

        core.set_version(version);
        core.bump_version().expect("version should advance");

        assert_eq!(core.version().value(), 4);
    }

    #[test]
    fn bump_version_returns_error_on_overflow() {
        let mut core = AggregateCore::<CounterId, CounterState, CounterEventPayload>::new();
        let max_version = AggregateVersion::try_from(i64::MAX).expect("version should be valid");
        core.set_version(max_version);

        let error = core
            .bump_version()
            .expect_err("overflow should return an error");

        assert!(matches!(error, AggregateVersionError::Overflow));
        assert_eq!(core.version(), max_version);
    }

    #[test]
    fn records_and_clears_uncommitted_events() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut core = AggregateCore::<CounterId, CounterState, CounterEventPayload>::new();
        let event = Event::new(
            aggregate_id,
            AggregateVersion::try_from(1).expect("version should be valid"),
            CounterEventPayload::Incremented { amount: 2 },
        );

        core.record_uncommitted_event(event.clone());

        let events = core.uncommitted_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], event);

        core.clear_uncommitted_events();

        assert!(core.uncommitted_events().is_empty());
    }
}
