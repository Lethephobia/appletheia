use std::ops::{Bound, RangeBounds};

use crate::aggregate::AggregateVersion;

/// Represents a range of aggregate versions.
///
/// This type preserves inclusive, exclusive, and unbounded bounds so callers
/// can express partial reads over an aggregate version sequence.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AggregateVersionRange {
    start: Bound<AggregateVersion>,
    end: Bound<AggregateVersion>,
}

impl AggregateVersionRange {
    /// Creates a range from the provided start and end bounds.
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

#[cfg(test)]
mod tests {
    use std::ops::{Bound, RangeBounds};

    use super::AggregateVersionRange;
    use crate::aggregate::AggregateVersion;

    #[test]
    fn new_preserves_start_and_end_bounds() {
        let start = Bound::Included(AggregateVersion::try_from(1).unwrap());
        let end = Bound::Excluded(AggregateVersion::try_from(5).unwrap());
        let range = AggregateVersionRange::new(start, end);

        assert_eq!(
            range.start_bound(),
            Bound::Included(&AggregateVersion::try_from(1).unwrap())
        );
        assert_eq!(
            range.end_bound(),
            Bound::Excluded(&AggregateVersion::try_from(5).unwrap())
        );
    }

    #[test]
    fn default_is_unbounded_on_both_sides() {
        let range = AggregateVersionRange::default();

        assert_eq!(range.start_bound(), Bound::Unbounded);
        assert_eq!(range.end_bound(), Bound::Unbounded);
    }

    #[test]
    fn range_supports_exclusive_and_inclusive_bounds() {
        let start = Bound::Excluded(AggregateVersion::try_from(2).unwrap());
        let end = Bound::Included(AggregateVersion::try_from(7).unwrap());
        let range = AggregateVersionRange::new(start, end);

        assert_eq!(
            range.start_bound(),
            Bound::Excluded(&AggregateVersion::try_from(2).unwrap())
        );
        assert_eq!(
            range.end_bound(),
            Bound::Included(&AggregateVersion::try_from(7).unwrap())
        );
    }
}
