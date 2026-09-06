use crate::command::CommandNameOwned;
use crate::request_context::MessageId;

use super::SagaStep;

/// Records one command dispatched by a persisted saga instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SagaDispatchedCommand<S: SagaStep> {
    pub message_id: MessageId,
    pub command_name: CommandNameOwned,
    pub step: S,
}
