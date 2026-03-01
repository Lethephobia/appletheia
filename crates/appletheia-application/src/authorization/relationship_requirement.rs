use super::{AggregateRef, RelationName};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RelationshipRequirement {
    None,
    Check {
        aggregate: AggregateRef,
        relation: RelationName,
    },
    All(Vec<RelationshipRequirement>),
    Any(Vec<RelationshipRequirement>),
    Not(Box<RelationshipRequirement>),
}
