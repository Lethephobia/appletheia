use serde::{Deserialize, Serialize};

use crate::event::AggregateTypeOwned;

use super::{AggregateRef, RelationName};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum RelationshipSubject {
    /// `<type>:<id>`
    Aggregate(AggregateRef),
    /// `<type>:*`
    Wildcard { aggregate_type: AggregateTypeOwned },
    /// `<type>:<id>#<relation>`
    AggregateSet {
        aggregate: AggregateRef,
        relation: RelationName,
    },
}
