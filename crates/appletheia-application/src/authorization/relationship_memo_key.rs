use super::{AggregateRef, RelationName};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(super) struct RelationshipMemoKey {
    pub(super) subject: AggregateRef,
    pub(super) aggregate: AggregateRef,
    pub(super) relation: RelationName,
}
