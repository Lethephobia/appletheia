use super::{ReadModelRelationshipKeyError, ReadModelResourceKey};

pub trait ReadModelRelationshipKey: Copy + Eq + Send + Sync {
    fn try_into_resource_key(self) -> Result<ReadModelResourceKey, ReadModelRelationshipKeyError>;
}
