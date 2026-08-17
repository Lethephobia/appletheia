/// Decides whether a complete candidate belongs to a list result.
///
/// Persistence readers must apply equivalent predicates when loading the initial snapshot.
pub trait ReadModelListCriteria: Send + Sync {
    /// Provides the materialized fields needed for membership evaluation.
    type Candidate;

    /// Returns `true` when the candidate satisfies every criterion.
    fn matches(&self, candidate: &Self::Candidate) -> bool;
}
