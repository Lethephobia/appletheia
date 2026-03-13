use serde::{Deserialize, Serialize};

use super::{AggregateRef, RelationNameOwned, RelationshipSubject};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Relationship {
    pub aggregate: AggregateRef,
    pub relation: RelationNameOwned,
    pub subject: RelationshipSubject,
}
