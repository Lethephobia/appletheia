use super::{ReadModelRelationshipData, ReadModelRelationshipError, ReadModelRelationshipKey};

/// One named relationship holding typed IDs rather than embedded resources.
pub trait ReadModelRelationship: Send + Sync {
    fn key(&self) -> ReadModelRelationshipKey;

    fn try_to_data(&self) -> Result<ReadModelRelationshipData, ReadModelRelationshipError>;
}
