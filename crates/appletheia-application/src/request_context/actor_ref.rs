use crate::authorization::AggregateRef;
use crate::request_context::{ActorRefError, Principal};

use serde::{Deserialize, Serialize};

/// Identifies the actor recorded in request context and emitted events.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ActorRef {
    /// Represents an unauthenticated external actor.
    Anonymous,
    /// Represents the framework or runtime itself.
    System,
    /// Represents a concrete aggregate subject acting on the request.
    Subject { subject: AggregateRef },
}

impl TryFrom<Principal> for ActorRef {
    type Error = ActorRefError;

    fn try_from(principal: Principal) -> Result<Self, Self::Error> {
        match principal {
            Principal::Anonymous => Ok(Self::Anonymous),
            Principal::System => Ok(Self::System),
            Principal::Authenticated { subject } => Ok(Self::Subject { subject }),
            Principal::Unavailable => Err(ActorRefError::PrincipalUnavailable),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};

    fn aggregate_ref() -> AggregateRef {
        AggregateRef {
            aggregate_type: AggregateTypeOwned::try_from("user").expect("valid aggregate type"),
            aggregate_id: AggregateIdValue::from(Uuid::nil()),
        }
    }

    #[test]
    fn serializes_and_deserializes_subject_variant() {
        let actor = ActorRef::Subject {
            subject: aggregate_ref(),
        };
        let value = serde_json::to_value(&actor).expect("serialize");

        assert_eq!(value["type"], serde_json::json!("subject"));
        assert_eq!(
            value["data"]["subject"]["aggregate_type"],
            serde_json::json!("user")
        );

        let round_trip: ActorRef = serde_json::from_value(value).expect("deserialize");

        assert_eq!(round_trip, actor);
    }

    #[test]
    fn preserves_simple_variants() {
        assert_eq!(ActorRef::Anonymous, ActorRef::Anonymous);
        assert_eq!(ActorRef::System, ActorRef::System);
    }

    #[test]
    fn serializes_simple_variants_with_type_only() {
        let anonymous = serde_json::to_value(ActorRef::Anonymous).expect("serialize anonymous");
        let system = serde_json::to_value(ActorRef::System).expect("serialize system");

        assert_eq!(anonymous, serde_json::json!({ "type": "anonymous" }));
        assert_eq!(system, serde_json::json!({ "type": "system" }));
    }

    #[test]
    fn try_from_authenticated_principal_builds_subject_actor() {
        let actor = ActorRef::try_from(Principal::Authenticated {
            subject: aggregate_ref(),
        })
        .expect("authenticated principal should convert");

        assert_eq!(
            actor,
            ActorRef::Subject {
                subject: aggregate_ref(),
            }
        );
    }

    #[test]
    fn try_from_unavailable_principal_returns_error() {
        let error =
            ActorRef::try_from(Principal::Unavailable).expect_err("unavailable principal fails");

        assert!(matches!(error, ActorRefError::PrincipalUnavailable));
    }
}
