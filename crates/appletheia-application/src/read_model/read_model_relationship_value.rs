use super::{ReadModelRelationshipData, ReadModelRelationshipValueError};

/// Typed relationship IDs converted into to-one or to-many linkage.
pub trait ReadModelRelationshipValue: Send + Sync {
    fn try_to_data(&self) -> Result<ReadModelRelationshipData, ReadModelRelationshipValueError>;
}
