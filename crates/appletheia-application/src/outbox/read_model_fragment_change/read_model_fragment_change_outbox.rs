use crate::messaging::PublishDispatchError;
use crate::read_model::{ReadModelFragmentChangeEnvelope, ReadModelFragmentChangeId};

use super::super::{OrderingKey, Outbox, OutboxLifecycle, OutboxState};

/// Adapts a durable source-fragment change envelope to the generic outbox relay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelFragmentChangeOutbox {
    pub change: ReadModelFragmentChangeEnvelope,
    pub state: OutboxState,
    pub last_error: Option<PublishDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox for ReadModelFragmentChangeOutbox {
    type Id = ReadModelFragmentChangeId;
    type Message = ReadModelFragmentChangeEnvelope;

    fn id(&self) -> Self::Id {
        self.change.change_id
    }

    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(&self.change.partition)
    }

    fn message(&self) -> &Self::Message {
        &self.change
    }

    fn state(&self) -> &OutboxState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut OutboxState {
        &mut self.state
    }

    fn last_error(&self) -> &Option<PublishDispatchError> {
        &self.last_error
    }

    fn last_error_mut(&mut self) -> &mut Option<PublishDispatchError> {
        &mut self.last_error
    }

    fn lifecycle(&self) -> &OutboxLifecycle {
        &self.lifecycle
    }

    fn lifecycle_mut(&mut self) -> &mut OutboxLifecycle {
        &mut self.lifecycle
    }
}
