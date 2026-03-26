use crate::authorization::AggregateRef;

use serde::{Deserialize, Serialize};

/// Identifies the actor recorded in request context and emitted events.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorRef {
    /// Represents an unauthenticated external actor.
    Anonymous,
    /// Represents the framework or runtime itself.
    System,
    /// Represents a concrete aggregate subject acting on the request.
    Subject { subject: AggregateRef },
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

        let round_trip: ActorRef =
            serde_json::from_value(serde_json::to_value(&actor).expect("serialize"))
                .expect("deserialize");

        assert_eq!(round_trip, actor);
    }

    #[test]
    fn preserves_simple_variants() {
        assert_eq!(ActorRef::Anonymous, ActorRef::Anonymous);
        assert_eq!(ActorRef::System, ActorRef::System);
    }
}
