pub mod correlation_id;
pub mod message_id;
pub mod request_context_access;

pub use correlation_id::CorrelationId;
pub use message_id::MessageId;
pub use request_context_access::RequestContextAccess;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestContext {
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
}
