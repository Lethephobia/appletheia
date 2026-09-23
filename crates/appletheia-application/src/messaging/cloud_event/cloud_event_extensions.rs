use std::collections::BTreeMap;

use super::{
    CloudEventAttributeValue, CloudEventExtensionName, CloudEventExtensionsError,
    CloudEventPartitionKey,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CloudEventExtensions {
    values: BTreeMap<CloudEventExtensionName, CloudEventAttributeValue>,
}

impl CloudEventExtensions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        name: CloudEventExtensionName,
        value: CloudEventAttributeValue,
    ) -> Result<Option<CloudEventAttributeValue>, CloudEventExtensionsError> {
        if name.as_str() == "partitionkey" {
            match &value {
                CloudEventAttributeValue::String(text) => {
                    CloudEventPartitionKey::new(text.as_str().to_owned())?;
                }
                _ => return Err(CloudEventExtensionsError::InvalidPartitionKey),
            }
        }
        Ok(self.values.insert(name, value))
    }

    pub fn get(&self, name: &CloudEventExtensionName) -> Option<&CloudEventAttributeValue> {
        self.values.get(name)
    }

    pub fn remove(&mut self, name: &CloudEventExtensionName) -> Option<CloudEventAttributeValue> {
        self.values.remove(name)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&CloudEventExtensionName, &CloudEventAttributeValue)> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub(super) fn insert_partition_key(&mut self, key: CloudEventPartitionKey) {
        self.values.insert(
            CloudEventExtensionName::partition_key(),
            CloudEventAttributeValue::String(key.into_attribute_string()),
        );
    }

    pub fn partition_key(&self) -> Option<CloudEventPartitionKey> {
        match self.get(&CloudEventExtensionName::partition_key()) {
            Some(CloudEventAttributeValue::String(value)) => {
                Some(CloudEventPartitionKey::from_validated_string(value))
            }
            _ => None,
        }
    }
}
