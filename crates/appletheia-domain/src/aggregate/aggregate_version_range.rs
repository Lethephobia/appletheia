use std::ops::{Bound, RangeBounds};

use crate::aggregate::AggregateVersion;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AggregateVersionRange {
    start: Bound<AggregateVersion>,
    end: Bound<AggregateVersion>,
}

impl AggregateVersionRange {
    pub fn new(start: Bound<AggregateVersion>, end: Bound<AggregateVersion>) -> Self {
        Self { start, end }
    }
}

impl Default for AggregateVersionRange {
    fn default() -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
}

impl RangeBounds<AggregateVersion> for AggregateVersionRange {
    fn start_bound(&self) -> Bound<&AggregateVersion> {
        match &self.start {
            Bound::Included(version) => Bound::Included(version),
            Bound::Excluded(version) => Bound::Excluded(version),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&AggregateVersion> {
        match &self.end {
            Bound::Included(version) => Bound::Included(version),
            Bound::Excluded(version) => Bound::Excluded(version),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
}
