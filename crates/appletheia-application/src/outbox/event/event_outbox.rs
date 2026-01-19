use super::EventOutboxId;
use crate::event::AppEvent;
use crate::outbox::{OrderingKey, Outbox};
use crate::outbox::{OutboxDispatchError, OutboxLifecycle, OutboxState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventOutbox {
    pub id: EventOutboxId,
    pub ordering_key: OrderingKey,
    pub event: AppEvent,
    pub state: OutboxState,
    pub last_error: Option<OutboxDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox for EventOutbox {
    type Id = EventOutboxId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn ordering_key(&self) -> &OrderingKey {
        &self.ordering_key
    }

    fn state(&self) -> &crate::outbox::OutboxState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut crate::outbox::OutboxState {
        &mut self.state
    }

    fn last_error(&self) -> &Option<crate::outbox::OutboxDispatchError> {
        &self.last_error
    }

    fn last_error_mut(&mut self) -> &mut Option<crate::outbox::OutboxDispatchError> {
        &mut self.last_error
    }

    fn lifecycle(&self) -> &crate::outbox::OutboxLifecycle {
        &self.lifecycle
    }

    fn lifecycle_mut(&mut self) -> &mut crate::outbox::OutboxLifecycle {
        &mut self.lifecycle
    }
}
