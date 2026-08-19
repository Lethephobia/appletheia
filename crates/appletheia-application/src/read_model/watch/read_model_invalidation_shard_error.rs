use std::num::NonZeroU32;

use thiserror::Error;

/// Reports an invalid fixed invalidation-shard selection.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ReadModelInvalidationShardError {
    #[error("read-model invalidation shard index {index} is outside shard count {count}")]
    OutOfRange { index: u32, count: NonZeroU32 },
}
