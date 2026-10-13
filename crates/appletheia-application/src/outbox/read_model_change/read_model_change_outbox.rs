use super::ReadModelChangeOutboxId;

use crate::messaging::PublishDispatchError;
use crate::outbox::Outbox;
use crate::outbox::{OutboxLifecycle, OutboxState};
use crate::read_model::ReadModelChangeEnvelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelChangeOutbox {
    pub id: ReadModelChangeOutboxId,
    pub change: ReadModelChangeEnvelope,
    pub state: OutboxState,
    pub last_error: Option<PublishDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox for ReadModelChangeOutbox {
    type Id = ReadModelChangeOutboxId;
    type Message = ReadModelChangeEnvelope;

    fn id(&self) -> Self::Id {
        self.id
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
