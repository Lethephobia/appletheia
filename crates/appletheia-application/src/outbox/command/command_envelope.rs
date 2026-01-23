use serde::{Deserialize, Serialize};

use crate::command::CommandName;
use crate::request_context::{CausationId, CorrelationId, MessageId};

use super::CommandPayload;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct CommandEnvelope<CN: CommandName> {
    pub command_name: CN,
    pub payload: CommandPayload,
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub causation_id: CausationId,
}
