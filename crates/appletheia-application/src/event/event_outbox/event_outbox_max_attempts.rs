use core::num::NonZeroU32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventOutboxMaxAttempts(NonZeroU32);

impl EventOutboxMaxAttempts {
    pub fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> NonZeroU32 {
        self.0
    }
}

impl Default for EventOutboxMaxAttempts {
    fn default() -> Self {
        // Safety: 1_000_000 is non-zero by definition.
        let value = NonZeroU32::new(1_000_000).expect("1_000_000 must be non-zero");
        Self::new(value)
    }
}
