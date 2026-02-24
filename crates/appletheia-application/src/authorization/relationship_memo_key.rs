use super::{AggregateRef, RelationName};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelationshipMemoKey {
    pub subject: AggregateRef,
    pub aggregate: AggregateRef,
    pub relation: RelationName,
}
