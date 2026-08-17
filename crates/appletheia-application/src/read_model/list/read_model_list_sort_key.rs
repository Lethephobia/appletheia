use std::cmp::Ordering;

/// Defines a stable cursor ordering for a materialized list candidate.
pub trait ReadModelListSortKey: Send + Sync {
    /// Provides the materialized fields used by this ordering.
    type Candidate;

    /// Identifies one position in the stable ordering.
    ///
    /// The cursor must contain a deterministic tie-breaker when the selected field is not unique.
    type Cursor;

    /// Creates the cursor corresponding to a candidate.
    fn cursor(&self, candidate: &Self::Candidate) -> Self::Cursor;

    /// Compares a candidate with a cursor using the key's ascending order.
    fn compare_to_cursor(&self, candidate: &Self::Candidate, cursor: &Self::Cursor) -> Ordering;
}
