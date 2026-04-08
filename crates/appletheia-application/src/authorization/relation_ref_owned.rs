use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::event::AggregateTypeOwned;

use super::{RelationNameOwned, RelationRef};

/// Owns a validated relation reference for runtime authorization data.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RelationRefOwned {
    pub aggregate_type: AggregateTypeOwned,
    pub relation_name: RelationNameOwned,
}

impl RelationRefOwned {
    pub fn new(aggregate_type: AggregateTypeOwned, relation_name: RelationNameOwned) -> Self {
        Self {
            aggregate_type,
            relation_name,
        }
    }
}

impl From<RelationRef> for RelationRefOwned {
    fn from(value: RelationRef) -> Self {
        Self::new(value.aggregate_type.into(), value.relation_name.into())
    }
}

impl Display for RelationRefOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.aggregate_type, self.relation_name)
    }
}
