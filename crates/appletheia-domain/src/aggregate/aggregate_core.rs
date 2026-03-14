use crate::event::{Event, EventPayload};

use super::{AggregateState, AggregateVersion, AggregateVersionError};

/// Stores the mutable bookkeeping shared by aggregate implementations.
///
/// The core tracks the current state, the latest aggregate version, and the
/// list of uncommitted events produced since the last persistence boundary.
#[derive(Clone, Debug)]
pub struct AggregateCore<S, P>
where
    S: AggregateState,
    P: EventPayload,
{
    state: Option<S>,
    version: AggregateVersion,
    uncommitted_events: Vec<Event<S::Id, P>>,
}

impl<S, P> AggregateCore<S, P>
where
    S: AggregateState,
    P: EventPayload,
{
    /// Creates an empty aggregate core with no state, version `0`, and no uncommitted events.
    pub fn new() -> Self {
        Self {
            state: None,
            version: AggregateVersion::new(),
            uncommitted_events: Vec::new(),
        }
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
    pub fn uncommitted_events(&self) -> &[Event<S::Id, P>] {
        &self.uncommitted_events
    }

    /// Records an uncommitted event.
    pub fn record_uncommitted_event(&mut self, event: Event<S::Id, P>) {
        self.uncommitted_events.push(event);
    }

    /// Removes all recorded uncommitted events.
    pub fn clear_uncommitted_events(&mut self) {
        self.uncommitted_events.clear();
    }
}

impl<S, P> Default for AggregateCore<S, P>
where
    S: AggregateState,
    P: EventPayload,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use appletheia_macros::{aggregate_id, aggregate_state, event_payload};
    use thiserror::Error;
    use uuid::Uuid;

    use super::AggregateCore;
    use crate::aggregate::{
        AggregateId, AggregateStateError, AggregateVersion, AggregateVersionError,
        UniqueConstraints, UniqueValuesError,
    };
    use crate::event::Event;

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

    #[aggregate_id(error = CounterIdError, validate = validate_counter_id)]
    struct CounterId(Uuid);

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),

        #[error(transparent)]
        UniqueValues(#[from] UniqueValuesError),
    }

    #[aggregate_state(error = CounterStateError)]
    struct CounterState {
        id: CounterId,
        count: i32,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {}

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[event_payload(error = CounterEventPayloadError)]
    enum CounterEventPayload {
        Incremented { amount: i32 },
    }

    #[test]
    fn new_initializes_empty_core() {
        let core = AggregateCore::<CounterState, CounterEventPayload>::new();

        assert!(core.state().is_none());
        assert_eq!(core.version(), AggregateVersion::new());
        assert!(core.uncommitted_events().is_empty());
    }

    #[test]
    fn default_matches_new() {
        let core = AggregateCore::<CounterState, CounterEventPayload>::default();

        assert!(core.state().is_none());
        assert_eq!(core.version(), AggregateVersion::new());
        assert!(core.uncommitted_events().is_empty());
    }

    #[test]
    fn state_accessors_read_and_update_state() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut core = AggregateCore::<CounterState, CounterEventPayload>::new();

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
        let mut core = AggregateCore::<CounterState, CounterEventPayload>::new();
        let version = AggregateVersion::try_from(3).expect("version should be valid");

        core.set_version(version);
        core.bump_version().expect("version should advance");

        assert_eq!(core.version().value(), 4);
    }

    #[test]
    fn bump_version_returns_error_on_overflow() {
        let mut core = AggregateCore::<CounterState, CounterEventPayload>::new();
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
        let mut core = AggregateCore::<CounterState, CounterEventPayload>::new();
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
