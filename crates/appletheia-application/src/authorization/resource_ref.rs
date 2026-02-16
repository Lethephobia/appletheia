use crate::event::{AggregateIdValue, AggregateTypeOwned};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ResourceRef {
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdValue,
}

