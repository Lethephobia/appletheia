use super::registered_part_route_map::RegisteredPartRouteMap;
use super::{ReadModelFragmentName, ReadModelPartPathResolver};

/// Holds one fragment-to-part route registered by a materialized part tree mapper.
pub(super) struct RegisteredPartRoute {
    pub(super) fragment_name: ReadModelFragmentName,
    pub(super) map: RegisteredPartRouteMap,
    pub(super) path_resolver: ReadModelPartPathResolver,
}
