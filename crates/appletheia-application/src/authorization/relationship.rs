use serde::{Deserialize, Serialize};

use super::{AggregateRef, RelationRefOwned, RelationshipSubject};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Relationship {
    pub aggregate: AggregateRef,
    pub relation: RelationRefOwned,
    pub subject: RelationshipSubject,
}
