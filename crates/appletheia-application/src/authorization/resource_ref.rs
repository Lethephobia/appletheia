use crate::event::{AggregateIdValue, AggregateTypeOwned};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResourceRef {
    Aggregate {
        aggregate_type: AggregateTypeOwned,
        aggregate_id: AggregateIdValue,
    },
    Custom {
        kind: String,
        id: Option<String>,
    },
}
