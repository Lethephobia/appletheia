use core::num::NonZeroU32;

/// Limits the number of source aggregate IDs returned by a reference index lookup page.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ReferenceIndexLookupPageSize(NonZeroU32);

impl ReferenceIndexLookupPageSize {
    pub fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> NonZeroU32 {
        self.0
    }

    pub fn as_i64(&self) -> i64 {
        self.value().get() as i64
    }

    pub fn as_usize(&self) -> usize {
        self.value().get() as usize
    }
}
