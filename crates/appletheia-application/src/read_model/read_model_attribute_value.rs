use super::{ReadModelAttributeValueError, SerializedReadModelAttributeValue};

/// A typed attribute value that serializes its contents without an enum variant tag.
pub trait ReadModelAttributeValue: Send + Sync {
    fn try_to_value(
        &self,
    ) -> Result<SerializedReadModelAttributeValue, ReadModelAttributeValueError>;
}
