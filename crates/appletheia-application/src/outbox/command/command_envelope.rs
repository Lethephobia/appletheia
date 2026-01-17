use serde::{Deserialize, Serialize};

use crate::command::{CommandHash, CommandNameOwned};
use crate::request_context::{CorrelationId, MessageId, RequestContext};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_name: CommandNameOwned,
    pub command_hash: CommandHash,
    pub payload: serde_json::Value,
    pub context: RequestContext,
    pub causation_id: MessageId,
    pub ordering_key: Option<String>,
}

impl CommandEnvelope {
    pub fn correlation_id(&self) -> CorrelationId {
        self.context.correlation_id
    }

    pub fn message_id(&self) -> MessageId {
        self.context.message_id
    }

    pub fn causation_id(&self) -> MessageId {
        self.causation_id
    }
}
