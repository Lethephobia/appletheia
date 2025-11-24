use core::num::NonZeroU32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct OutboxBatchSize(NonZeroU32);

impl OutboxBatchSize {
    pub fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub fn value(self) -> NonZeroU32 {
        self.0
    }

    pub fn as_i64(self) -> i64 {
        self.value().get() as i64
    }
}
