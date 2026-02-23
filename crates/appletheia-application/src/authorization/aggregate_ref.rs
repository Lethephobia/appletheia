use appletheia_domain::{Aggregate, AggregateId};
use serde::{Deserialize, Serialize};

use crate::event::{AggregateIdValue, AggregateTypeOwned};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AggregateRef {
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdValue,
}

impl AggregateRef {
    pub fn from_aggregate<A: Aggregate>(id: A::Id) -> Self {
        Self {
            aggregate_type: AggregateTypeOwned::from(A::TYPE),
            aggregate_id: AggregateIdValue::from(id.value()),
        }
    }
}
