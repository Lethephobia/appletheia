use crate::event::{Event, EventPayload};

use super::{AggregateState, AggregateVersion, AggregateVersionError};

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
    pub fn new() -> Self {
        Self {
            state: None,
            version: AggregateVersion::new(),
            uncommitted_events: Vec::new(),
        }
    }

    pub fn state(&self) -> Option<&S> {
        self.state.as_ref()
    }

    pub fn state_mut(&mut self) -> Option<&mut S> {
        self.state.as_mut()
    }

    pub fn set_state(&mut self, state: Option<S>) {
        self.state = state;
    }

    pub fn version(&self) -> AggregateVersion {
        self.version
    }

    pub fn set_version(&mut self, version: AggregateVersion) {
        self.version = version;
    }

    pub(crate) fn bump_version(&mut self) -> Result<(), AggregateVersionError> {
        let next_version = self.version.try_next()?;
        self.version = next_version;
        Ok(())
    }

    pub fn uncommitted_events(&self) -> &[Event<S::Id, P>] {
        &self.uncommitted_events
    }

    pub fn record_uncommitted_event(&mut self, event: Event<S::Id, P>) {
        self.uncommitted_events.push(event);
    }

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
