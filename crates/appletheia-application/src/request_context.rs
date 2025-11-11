pub mod correlation_id;
pub mod message_id;

pub use correlation_id::CorrelationId;
pub use message_id::MessageId;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestContext {
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
}
