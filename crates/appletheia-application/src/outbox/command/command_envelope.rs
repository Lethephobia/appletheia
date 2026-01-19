use serde::{Deserialize, Serialize};

use crate::command::CommandNameOwned;
use crate::request_context::{CausationId, CorrelationId, MessageId};

use super::CommandPayload;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_name: CommandNameOwned,
    pub payload: CommandPayload,
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub causation_id: CausationId,
}
