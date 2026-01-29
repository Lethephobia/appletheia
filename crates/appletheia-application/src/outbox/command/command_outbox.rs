use super::{CommandEnvelope, CommandOutboxId};
use crate::massaging::PublishDispatchError;
use crate::outbox::{OrderingKey, Outbox, OutboxLifecycle, OutboxState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandOutbox {
    pub id: CommandOutboxId,
    pub sequence: i64,
    pub command: CommandEnvelope,
    pub state: OutboxState,
    pub last_error: Option<PublishDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox for CommandOutbox {
    type Id = CommandOutboxId;
    type Message = CommandEnvelope;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(self.command.correlation_id)
    }

    fn message(&self) -> &Self::Message {
        &self.command
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
