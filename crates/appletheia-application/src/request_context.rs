pub mod actor_ref;
pub mod causation_id;
pub mod correlation_id;
pub mod message_id;
pub mod principal;
pub mod request_context_access;
pub mod subject_id;
pub mod subject_id_error;
pub mod subject_kind;
pub mod subject_kind_error;
pub mod subject_ref;
pub mod tenant_id;
pub mod tenant_id_error;

pub use actor_ref::ActorRef;
pub use causation_id::CausationId;
pub use correlation_id::CorrelationId;
pub use message_id::MessageId;
pub use principal::Principal;
pub use request_context_access::RequestContextAccess;
pub use subject_id::SubjectId;
pub use subject_id_error::SubjectIdError;
pub use subject_kind::SubjectKind;
pub use subject_kind_error::SubjectKindError;
pub use subject_ref::SubjectRef;
pub use tenant_id::TenantId;
pub use tenant_id_error::TenantIdError;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestContext {
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub actor: ActorRef,

    #[serde(skip, default)]
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
