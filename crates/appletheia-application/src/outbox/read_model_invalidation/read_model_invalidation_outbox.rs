use crate::messaging::PublishDispatchError;
use crate::read_model::{ReadModelInvalidationEnvelope, ReadModelInvalidationId};

use super::super::{OrderingKey, Outbox, OutboxLifecycle, OutboxState};

/// Adapts a durable read-model invalidation to the generic outbox relay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelInvalidationOutbox {
    pub invalidation: ReadModelInvalidationEnvelope,
    pub state: OutboxState,
    pub last_error: Option<PublishDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox for ReadModelInvalidationOutbox {
    type Id = ReadModelInvalidationId;
    type Message = ReadModelInvalidationEnvelope;

    fn id(&self) -> Self::Id {
        self.invalidation.invalidation_id
    }

    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(self.invalidation.correlation_id)
    }

    fn message(&self) -> &Self::Message {
        &self.invalidation
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
