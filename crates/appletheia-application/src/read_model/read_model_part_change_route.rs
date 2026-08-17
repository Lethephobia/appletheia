use super::{
    ReadModelFragment, ReadModelFragmentName, ReadModelPartChange, ReadModelPartChangeError,
    ReadModelPartPathResolver, SerializedReadModelFragmentChange,
};

pub(super) type MapFragmentChange =
    fn(
        &SerializedReadModelFragmentChange,
        ReadModelPartPathResolver,
    ) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError>;

/// Declares a part-change route that cannot be derived from a read model's part tree.
#[derive(Copy, Clone)]
pub struct ReadModelPartChangeRoute {
    pub(super) fragment_name: ReadModelFragmentName,
    pub(super) map: MapFragmentChange,
}

impl ReadModelPartChangeRoute {
    /// Creates an explicit route from one source fragment.
    pub const fn from_fragment<F>(map: MapFragmentChange) -> Self
    where
        F: ReadModelFragment,
    {
        Self {
            fragment_name: F::NAME,
            map,
        }
    }
}
