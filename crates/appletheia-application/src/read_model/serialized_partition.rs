use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::json::CanonicalJson;

use super::{ReadModelFragment, ReadModelFragmentNameOwned, SerializedPartitionError};

/// Stores a source-fragment partition in transport-neutral JSON form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerializedPartition(serde_json::Value);

impl SerializedPartition {
    /// Returns the serialized partition key.
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }

    /// Returns a canonical JSON representation whose object keys are sorted recursively.
    pub fn canonical_json(&self) -> CanonicalJson {
        CanonicalJson::from_value(&self.0)
    }

    /// Serializes a physical fragment key with its fragment identity.
    pub fn try_from_fragment_key<F>(key: &F::Key) -> Result<Self, SerializedPartitionError>
    where
        F: ReadModelFragment,
    {
        let serialized_key =
            serde_json::to_value(key).map_err(SerializedPartitionError::SerializeKey)?;
        let partition = serde_json::json!({
            "fragment_name": ReadModelFragmentNameOwned::from(F::NAME),
            "key": serialized_key,
        });

        Self::try_from(partition)
    }

    /// Deserializes a physical fragment key from this partition.
    pub fn try_fragment_key<F>(&self) -> Result<F::Key, SerializedPartitionError>
    where
        F: ReadModelFragment,
    {
        let object = self
            .0
            .as_object()
            .ok_or(SerializedPartitionError::InvalidShape)?;
        let fragment_name = object
            .get("fragment_name")
            .and_then(serde_json::Value::as_str)
            .ok_or(SerializedPartitionError::InvalidShape)?;
        if fragment_name != F::NAME.value() {
            return Err(SerializedPartitionError::FragmentMismatch {
                expected: F::NAME.value().to_owned(),
                actual: fragment_name.to_owned(),
            });
        }
        let key = object
            .get("key")
            .ok_or(SerializedPartitionError::InvalidShape)?;

        serde_json::from_value(key.clone()).map_err(SerializedPartitionError::DeserializeKey)
    }
}

impl Hash for SerializedPartition {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.canonical_json().hash(state);
    }
}

impl TryFrom<serde_json::Value> for SerializedPartition {
    type Error = SerializedPartitionError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(SerializedPartitionError::NullPartition);
        }
        Ok(Self(value))
    }
}

impl From<SerializedPartition> for serde_json::Value {
    fn from(value: SerializedPartition) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rejects_null() {
        let result = serde_json::from_str::<SerializedPartition>("null");

        assert!(result.is_err());
    }

    #[test]
    fn equal_object_partitions_have_the_same_hash_regardless_of_key_order() {
        use std::collections::HashSet;

        let first = serde_json::from_str::<SerializedPartition>(
            r#"{"organization_id":"one","type":"organization"}"#,
        )
        .expect("first partition should be valid");
        let second = serde_json::from_str::<SerializedPartition>(
            r#"{"type":"organization","organization_id":"one"}"#,
        )
        .expect("second partition should be valid");
        let partitions = HashSet::from([first]);

        assert!(partitions.contains(&second));
    }
}
