use super::RelationRefOwned;
use crate::aggregate::AggregateRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelationshipMemoKey {
    pub subject: AggregateRef,
    pub target: AggregateRef,
    pub relation: RelationRefOwned,
}
