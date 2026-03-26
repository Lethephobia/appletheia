use crate::authorization::AggregateRef;

/// Represents the principal available to authorization logic for the current request.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum Principal {
    /// Indicates that principal information is not available in the current context.
    #[default]
    Unavailable,
    /// Represents an unauthenticated caller.
    Anonymous,
    /// Represents the framework or runtime itself.
    System,
    /// Represents an authenticated aggregate subject.
    Authenticated { subject: AggregateRef },
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
    fn default_is_unavailable() {
        assert_eq!(Principal::default(), Principal::Unavailable);
    }

    #[test]
    fn authenticated_variant_preserves_subject() {
        let subject = aggregate_ref();
        let principal = Principal::Authenticated {
            subject: subject.clone(),
        };

        assert_eq!(principal, Principal::Authenticated { subject });
    }
}
