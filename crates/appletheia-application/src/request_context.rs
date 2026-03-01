pub mod actor_ref;
pub mod causation_id;
pub mod correlation_id;
pub mod message_id;
pub mod principal;

pub use actor_ref::ActorRef;
pub use causation_id::CausationId;
pub use correlation_id::CorrelationId;
pub use message_id::MessageId;
pub use principal::Principal;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestContext {
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub actor: ActorRef,

    #[serde(skip)]
    pub principal: Principal,
}

impl RequestContext {
    pub fn new(
        correlation_id: CorrelationId,
        message_id: MessageId,
        actor: ActorRef,
        principal: Principal,
    ) -> Self {
        Self {
            correlation_id,
            message_id,
            actor,
            principal,
        }
    }
}
