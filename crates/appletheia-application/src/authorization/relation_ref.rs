use std::fmt::{self, Display};

use appletheia_domain::AggregateType;

use super::RelationName;

/// Identifies a statically-defined relation on a specific aggregate type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelationRef {
    pub aggregate_type: AggregateType,
    pub relation_name: RelationName,
}

impl RelationRef {
    pub const fn new(aggregate_type: AggregateType, relation_name: RelationName) -> Self {
        Self {
            aggregate_type,
            relation_name,
        }
    }
}

impl Display for RelationRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.aggregate_type, self.relation_name)
    }
}
