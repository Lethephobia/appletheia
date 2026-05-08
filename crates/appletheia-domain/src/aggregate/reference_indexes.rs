use super::ReferenceEntries;

/// Describes reference indexes derived from aggregate state.
pub trait ReferenceIndexes<E>: Send + Sync
where
    E: std::error::Error + Send + Sync + 'static,
{
    /// Returns the reference-index definitions derived from the current state.
    fn reference_entries(&self) -> Result<ReferenceEntries, E> {
        Ok(ReferenceEntries::new())
    }
}
