use super::EventOutboxId;
use appletheia_domain::AggregateType;

use crate::event::AppEvent;
use crate::outbox::{OrderingKey, Outbox};
use crate::outbox::{OutboxDispatchError, OutboxLifecycle, OutboxState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventOutbox<AT: AggregateType> {
    pub id: EventOutboxId,
    pub ordering_key: OrderingKey,
    pub event: AppEvent<AT>,
    pub state: OutboxState,
    pub last_error: Option<OutboxDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl<AT: AggregateType> Outbox for EventOutbox<AT> {
    type Id = EventOutboxId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn ordering_key(&self) -> &OrderingKey {
        &self.ordering_key
    }

    fn state(&self) -> &OutboxState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut OutboxState {
        &mut self.state
    }

    fn last_error(&self) -> &Option<OutboxDispatchError> {
        &self.last_error
    }

    fn last_error_mut(&mut self) -> &mut Option<OutboxDispatchError> {
        &mut self.last_error
    }

    fn lifecycle(&self) -> &OutboxLifecycle {
        &self.lifecycle
    }

    fn lifecycle_mut(&mut self) -> &mut OutboxLifecycle {
        &mut self.lifecycle
    }
}
