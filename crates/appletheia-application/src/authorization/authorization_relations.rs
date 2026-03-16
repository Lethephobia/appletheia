use super::AuthorizationTypeDefinition;
use appletheia_domain::AggregateType;

/// Groups multiple statically-typed authorization relations for a single aggregate.
///
/// This trait is intended for in-memory, compile-time configuration of
/// authorization models, where an aggregate's full set of relations is defined
/// in one place.
pub trait AuthorizationRelations {
    const AGGREGATE_TYPE: AggregateType;

    /// Builds the full authorization type definition for this aggregate.
    fn build(&self) -> AuthorizationTypeDefinition;
}
