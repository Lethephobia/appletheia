#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProcessedOutboxCount(u32);

impl ProcessedOutboxCount {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn value(&self) -> u32 {
        self.0
    }

    pub fn from_usize_saturating(value: usize) -> Self {
        Self(value.min(u32::MAX as usize) as u32)
    }
}
