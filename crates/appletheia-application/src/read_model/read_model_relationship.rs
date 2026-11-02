use super::{ReadModelRelationshipError, ReadModelResourceRelationship};

pub trait ReadModelRelationship: Send + Sync {
    fn try_into_resource_relationship(
        self,
    ) -> Result<ReadModelResourceRelationship, ReadModelRelationshipError>;
}
