use std::num::NonZeroU32;

use thiserror::Error;

/// Reports a shard index outside its configured fixed shard set.
#[derive(Debug, Error)]
pub enum ReadModelFragmentChangeShardError {
    #[error("read model fragment change shard {index} is outside shard count {count}")]
    OutOfRange { index: u32, count: NonZeroU32 },
}
