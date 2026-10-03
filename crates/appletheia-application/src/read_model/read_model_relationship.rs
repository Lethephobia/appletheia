use super::{ReadModelRelationshipError, ReadModelResourceRelationship};

pub trait ReadModelRelationship: Send + Sync {
    fn try_to_resource_relationship(
        &self,
    ) -> Result<ReadModelResourceRelationship, ReadModelRelationshipError>;
}
