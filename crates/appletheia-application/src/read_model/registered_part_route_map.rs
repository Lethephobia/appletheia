use super::read_model_part_change_route::MapFragmentChange;
use super::read_model_part_tree::MapPartTreeFragmentChange;
use super::{
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartPathResolver,
    SerializedReadModelFragmentChange,
};

/// Selects the mapping function used by one registered part route.
pub(super) enum RegisteredPartRouteMap {
    Tree(MapPartTreeFragmentChange),
    Explicit(MapFragmentChange),
}

impl RegisteredPartRouteMap {
    pub(super) fn map(
        &self,
        change: &SerializedReadModelFragmentChange,
        path_resolver: ReadModelPartPathResolver,
    ) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
        match self {
            Self::Tree(map) => map(change, path_resolver),
            Self::Explicit(map) => map(change, path_resolver),
        }
    }
}
