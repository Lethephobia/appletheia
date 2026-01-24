use serde::{Deserialize, Serialize};

use crate::command::CommandNameOwned;
use crate::request_context::{CausationId, CorrelationId, MessageId};

use super::SerializedCommand;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_name: CommandNameOwned,
    pub command: SerializedCommand,
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub causation_id: CausationId,
}
