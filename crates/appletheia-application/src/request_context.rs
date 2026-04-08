pub mod actor_ref;
pub mod actor_ref_error;
pub mod causation_id;
pub mod correlation_id;
pub mod message_id;
pub mod principal;
pub mod request_context_error;

pub use actor_ref::ActorRef;
pub use actor_ref_error::ActorRefError;
pub use causation_id::CausationId;
pub use correlation_id::CorrelationId;
pub use message_id::MessageId;
pub use principal::Principal;
pub use request_context_error::RequestContextError;

use serde::{Deserialize, Serialize};

/// Carries request-scoped metadata through the application pipeline.
///
/// `principal` is kept out of serialized forms because it represents ambient runtime
/// authentication context rather than transport metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestContext {
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub actor: ActorRef,

    #[serde(skip)]
    pub principal: Principal,
}

impl RequestContext {
    /// Creates a request context from correlation, message, and principal data.
    pub fn new(
        correlation_id: CorrelationId,
        message_id: MessageId,
        principal: Principal,
    ) -> Result<Self, RequestContextError> {
        let actor = ActorRef::try_from(principal.clone()).map_err(RequestContextError::ActorRef)?;

        Ok(Self {
            correlation_id,
            message_id,
            actor,
            principal,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use uuid::Uuid;

    use super::*;
    use crate::authorization::AggregateRef;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};

    fn aggregate_ref() -> AggregateRef {
        AggregateRef {
            aggregate_type: AggregateTypeOwned::try_from("user").expect("valid aggregate type"),
            aggregate_id: AggregateIdValue::from(Uuid::nil()),
        }
    }

    #[test]
    fn new_stores_all_fields() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::from(Uuid::now_v7());
        let principal = Principal::Authenticated {
            subject: aggregate_ref(),
        };

        let request_context = RequestContext::new(correlation_id, message_id, principal.clone())
            .expect("request context should be valid");

        assert_eq!(request_context.correlation_id, correlation_id);
        assert_eq!(request_context.message_id, message_id);
        assert_eq!(
            request_context.actor,
            ActorRef::Subject {
                subject: aggregate_ref(),
            }
        );
        assert_eq!(request_context.principal, principal);
    }

    #[test]
    fn serialization_skips_principal_and_deserialization_uses_default() {
        let request_context = RequestContext::new(
            CorrelationId::from(Uuid::nil()),
            MessageId::from(Uuid::now_v7()),
            Principal::Authenticated {
                subject: aggregate_ref(),
            },
        )
        .expect("request context should be valid");

        let serialized = serde_json::to_value(&request_context).expect("serialize request context");
        assert_eq!(
            serialized,
            json!({
                "correlation_id": request_context.correlation_id.value(),
                "message_id": request_context.message_id.value(),
                "actor": {
                    "Subject": {
                        "subject": {
                            "aggregate_type": "user",
                            "aggregate_id": Uuid::nil(),
                        }
                    }
                }
            })
        );

        let deserialized: RequestContext =
            serde_json::from_value(serialized).expect("deserialize request context");
        assert_eq!(deserialized.principal, Principal::Unavailable);
    }

    #[test]
    fn new_returns_error_for_unavailable_principal() {
        let error = RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::from(Uuid::now_v7()),
            Principal::Unavailable,
        )
        .expect_err("unavailable principal should fail");

        assert!(matches!(
            error,
            RequestContextError::ActorRef(ActorRefError::PrincipalUnavailable)
        ));
    }
}
