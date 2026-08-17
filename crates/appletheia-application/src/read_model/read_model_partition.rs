use super::{ReadModelFragment, SerializedPartition, SerializedPartitionError};

/// Identifies one physical fragment partition before it crosses a serialization boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelPartition<K> {
    key: K,
}

impl<K> ReadModelPartition<K> {
    /// Creates a partition from one physical fragment key.
    pub fn new(key: K) -> Self {
        Self { key }
    }

    /// Returns the physical fragment key.
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Serializes this partition for persistence or transport.
    pub fn try_into_serialized<F>(self) -> Result<SerializedPartition, SerializedPartitionError>
    where
        F: ReadModelFragment<Key = K>,
    {
        SerializedPartition::try_from_fragment_key::<F>(&self.key)
    }
}
