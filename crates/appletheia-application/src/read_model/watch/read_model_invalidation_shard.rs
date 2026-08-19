use std::num::NonZeroU32;

use sha2::{Digest, Sha256};

use crate::messaging::Selector;
use crate::read_model::ReadModelInvalidationEnvelope;

use super::ReadModelInvalidationShardError;

/// Selects one fixed transport shard for a read-model invalidation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReadModelInvalidationShard {
    index: u32,
    count: NonZeroU32,
}

impl ReadModelInvalidationShard {
    pub const ATTRIBUTE_NAME: &'static str = "shard";

    pub fn new(index: u32, count: NonZeroU32) -> Result<Self, ReadModelInvalidationShardError> {
        if index >= count.get() {
            return Err(ReadModelInvalidationShardError::OutOfRange { index, count });
        }
        Ok(Self { index, count })
    }

    pub fn for_envelope(envelope: &ReadModelInvalidationEnvelope, count: NonZeroU32) -> Self {
        let digest = Sha256::digest(envelope.invalidation_id.value().as_bytes());
        let hash = u64::from_be_bytes(
            digest[..8]
                .try_into()
                .expect("SHA-256 digest always has at least eight bytes"),
        );
        let index = (hash % u64::from(count.get())) as u32;
        Self { index, count }
    }

    pub const fn index(self) -> u32 {
        self.index
    }

    pub fn attribute_value(self) -> String {
        self.index.to_string()
    }

    pub fn ordering_key(self) -> String {
        format!("read-model-invalidation-shard-{}", self.index)
    }
}

impl Selector<ReadModelInvalidationEnvelope> for ReadModelInvalidationShard {
    fn matches(&self, message: &ReadModelInvalidationEnvelope) -> bool {
        Self::for_envelope(message, self.count).index == self.index
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;

    #[test]
    fn rejects_an_index_outside_the_fixed_shard_count() {
        let count = NonZeroU32::new(64).expect("shard count should be nonzero");
        let error = ReadModelInvalidationShard::new(64, count)
            .expect_err("index equal to count should be rejected");

        assert_eq!(
            error,
            ReadModelInvalidationShardError::OutOfRange { index: 64, count }
        );
    }
}
