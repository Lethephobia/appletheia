use super::{AggregateRef, RelationRefOwned};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelationshipMemoKey {
    pub subject: AggregateRef,
    pub aggregate: AggregateRef,
    pub relation: RelationRefOwned,
}
