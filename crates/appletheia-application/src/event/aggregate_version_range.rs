use std::ops::{Bound, RangeBounds};

use crate::aggregate::AggregateVersion;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AggregateVersionRange {
    from: Bound<AggregateVersion>,
    to: Bound<AggregateVersion>,
}

impl AggregateVersionRange {
    pub fn new(from: Bound<AggregateVersion>, to: Bound<AggregateVersion>) -> Self {
        Self { from, to }
    }
}

impl Default for AggregateVersionRange {
    fn default() -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
}

impl RangeBounds<AggregateVersion> for AggregateVersionRange {
    fn start_bound(&self) -> Bound<&AggregateVersion> {
        match &self.from {
            Bound::Included(version) => Bound::Included(version),
            Bound::Excluded(version) => Bound::Excluded(version),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&AggregateVersion> {
        match &self.to {
            Bound::Included(version) => Bound::Included(version),
            Bound::Excluded(version) => Bound::Excluded(version),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
}
