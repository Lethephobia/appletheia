use core::num::NonZeroU32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxMaxAttempts(NonZeroU32);

impl OutboxMaxAttempts {
    pub fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> NonZeroU32 {
        self.0
    }
}

impl Default for OutboxMaxAttempts {
    fn default() -> Self {
        Self(NonZeroU32::MIN.saturating_add(999_999))
    }
}
