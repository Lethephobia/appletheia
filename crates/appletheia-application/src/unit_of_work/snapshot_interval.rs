use core::num::NonZeroU32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SnapshotInterval(NonZeroU32);

impl SnapshotInterval {
    pub fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub fn value(self) -> NonZeroU32 {
        self.0
    }
}
