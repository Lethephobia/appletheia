use super::{ReadModelAttributeValueError, ReadModelResourceAttributeValue};

/// A typed attribute value that serializes its contents without an enum variant tag.
pub trait ReadModelAttributeValue: Send + Sync {
    fn try_to_resource_attribute_value(
        &self,
    ) -> Result<ReadModelResourceAttributeValue, ReadModelAttributeValueError>;
}
