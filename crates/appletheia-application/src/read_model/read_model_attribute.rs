use super::{ReadModelAttributeError, ReadModelAttributeKey, ReadModelAttributeValue};

/// One typed attribute variant. Implementations serialize its contained value only.
pub trait ReadModelAttribute: Send + Sync {
    fn key(&self) -> ReadModelAttributeKey;

    fn try_to_value(&self) -> Result<ReadModelAttributeValue, ReadModelAttributeError>;
}
