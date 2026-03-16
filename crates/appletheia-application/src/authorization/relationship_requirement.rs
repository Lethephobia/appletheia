use super::{AggregateRef, RelationNameOwned};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RelationshipRequirement {
    Check {
        aggregate: AggregateRef,
        relation: RelationNameOwned,
    },
    All(Vec<RelationshipRequirement>),
    Any(Vec<RelationshipRequirement>),
    Not(Box<RelationshipRequirement>),
}
