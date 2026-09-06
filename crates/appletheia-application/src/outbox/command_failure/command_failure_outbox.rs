use crate::command::CommandFailureEnvelope;
use crate::messaging::PublishDispatchError;
use crate::outbox::{OrderingKey, Outbox, OutboxLifecycle, OutboxState};

use super::CommandFailureOutboxId;

/// Relays one terminal command-failure notification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandFailureOutbox {
    pub id: CommandFailureOutboxId,
    pub sequence: i64,
    pub failure: CommandFailureEnvelope,
    pub state: OutboxState,
    pub last_error: Option<PublishDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox for CommandFailureOutbox {
    type Id = CommandFailureOutboxId;
    type Message = CommandFailureEnvelope;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(self.failure.correlation_id)
    }

    fn message(&self) -> &Self::Message {
        &self.failure
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
