use super::{AggregateRef, RelationNameOwned};

/// Describes relationship checks required for authorization.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RelationshipRequirement {
    /// Requires the principal to satisfy a relation on a specific aggregate.
    Check {
        /// The aggregate on which the relation is evaluated.
        aggregate: AggregateRef,
        /// The relation the principal must satisfy on the aggregate.
        relation: RelationNameOwned,
    },
    /// Requires all contained relationship requirements to be satisfied.
    All(Vec<RelationshipRequirement>),
    /// Requires at least one contained relationship requirement to be satisfied.
    Any(Vec<RelationshipRequirement>),
    /// Requires the contained relationship requirement to be unsatisfied.
    Not(Box<RelationshipRequirement>),
}
