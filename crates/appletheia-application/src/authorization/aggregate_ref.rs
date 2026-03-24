use appletheia_domain::{Aggregate, AggregateId};
use serde::{Deserialize, Serialize};

use super::AggregateRefError;
use crate::event::{AggregateIdValue, AggregateTypeOwned};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AggregateRef {
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdValue,
}

impl AggregateRef {
    pub fn try_from_aggregate<A: Aggregate>(value: &A) -> Result<Self, AggregateRefError> {
        let Some(id) = value.aggregate_id() else {
            return Err(AggregateRefError::MissingAggregateId);
        };

        Ok(Self {
            aggregate_type: AggregateTypeOwned::from(A::TYPE),
            aggregate_id: AggregateIdValue::from(id.value()),
        })
    }
}
