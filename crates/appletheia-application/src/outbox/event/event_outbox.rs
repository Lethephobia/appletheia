use super::EventOutboxId;

use crate::event::EventEnvelope;
use crate::messaging::PublishDispatchError;
use crate::outbox::{OrderingKey, Outbox};
use crate::outbox::{OutboxLifecycle, OutboxState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventOutbox {
    pub id: EventOutboxId,
    pub event: EventEnvelope,
    pub state: OutboxState,
    pub last_error: Option<PublishDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox for EventOutbox {
    type Id = EventOutboxId;
    type Message = EventEnvelope;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from((&self.event.aggregate_type, &self.event.aggregate_id))
    }

    fn message(&self) -> &Self::Message {
        &self.event
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
