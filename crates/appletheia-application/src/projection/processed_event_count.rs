#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProcessedEventCount(u64);

impl ProcessedEventCount {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }

    pub const fn saturating_add(self, delta: u64) -> Self {
        Self(self.0.saturating_add(delta))
    }
}
