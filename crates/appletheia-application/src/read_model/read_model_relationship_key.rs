use super::{ReadModelRelationshipKeyError, ReadModelResourceKey};

pub trait ReadModelRelationshipKey: Copy + Eq + Send + Sync {
    fn try_to_resource_key(&self) -> Result<ReadModelResourceKey, ReadModelRelationshipKeyError>;
}
