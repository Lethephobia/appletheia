use super::{ReadModelAttributeKeyError, ReadModelResourceKey};

pub trait ReadModelAttributeKey: Copy + Eq + Send + Sync {
    fn try_to_resource_key(&self) -> Result<ReadModelResourceKey, ReadModelAttributeKeyError>;
}
