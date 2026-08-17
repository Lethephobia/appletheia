use std::num::NonZeroU32;

use sha2::{Digest, Sha256};

use crate::messaging::Selector;
use crate::read_model::{ReadModelFragmentChangeEnvelope, SerializedPartition};

use super::ReadModelFragmentChangeShardError;

/// Identifies one member of the fixed fragment-change transport shard set.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReadModelFragmentChangeShard {
    index: u32,
    count: NonZeroU32,
}

impl ReadModelFragmentChangeShard {
    pub const ATTRIBUTE_NAME: &'static str = "shard";

    pub fn new(index: u32, count: NonZeroU32) -> Result<Self, ReadModelFragmentChangeShardError> {
        if index >= count.get() {
            return Err(ReadModelFragmentChangeShardError::OutOfRange { index, count });
        }
        Ok(Self { index, count })
    }

    pub fn for_envelope(envelope: &ReadModelFragmentChangeEnvelope, count: NonZeroU32) -> Self {
        let index = Self::index_for_partition(&envelope.partition, count);
        Self { index, count }
    }

    fn index_for_partition(partition: &SerializedPartition, count: NonZeroU32) -> u32 {
        let canonical_partition = partition.canonical_json();
        let digest = Sha256::digest(canonical_partition.as_str().as_bytes());
        let hash = digest
            .iter()
            .take(std::mem::size_of::<u64>())
            .fold(0_u64, |value, byte| (value << 8) | u64::from(*byte));
        (hash % u64::from(count.get())) as u32
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn count(&self) -> NonZeroU32 {
        self.count
    }

    pub fn attribute_value(&self) -> String {
        self.index.to_string()
    }

    pub fn ordering_key(&self) -> String {
        format!("read-model-fragment-shard:{}", self.index)
    }
}

impl Selector<ReadModelFragmentChangeEnvelope> for ReadModelFragmentChangeShard {
    fn matches(&self, message: &ReadModelFragmentChangeEnvelope) -> bool {
        Self::for_envelope(message, self.count) == *self
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use crate::read_model::SerializedPartition;

    use super::ReadModelFragmentChangeShard;

    #[test]
    fn new_rejects_an_index_outside_the_fixed_shard_set() {
        let count = NonZeroU32::new(64).expect("shard count should be nonzero");

        let error = ReadModelFragmentChangeShard::new(64, count)
            .expect_err("index equal to count should be rejected");

        assert_eq!(
            error.to_string(),
            "read model fragment change shard 64 is outside shard count 64"
        );
    }

    #[test]
    fn transport_values_are_derived_from_the_same_shard_identity() {
        let shard = ReadModelFragmentChangeShard::new(
            7,
            NonZeroU32::new(64).expect("shard count should be nonzero"),
        )
        .expect("shard should be valid");

        assert_eq!(ReadModelFragmentChangeShard::ATTRIBUTE_NAME, "shard");
        assert_eq!(shard.attribute_value(), "7");
        assert_eq!(shard.ordering_key(), "read-model-fragment-shard:7");
    }

    #[test]
    fn assigns_partitions_with_a_stable_sha256_hash() {
        let partition = SerializedPartition::try_from(serde_json::json!({
            "fragment_name": "user",
            "key": "019feb8c-d525-7b01-91d5-018b73dad7a7",
        }))
        .expect("partition should be valid");
        let count = NonZeroU32::new(64).expect("shard count should be nonzero");

        let index = ReadModelFragmentChangeShard::index_for_partition(&partition, count);

        assert_eq!(index, 43);
    }
}
