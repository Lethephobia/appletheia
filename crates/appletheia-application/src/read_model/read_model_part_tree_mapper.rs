use super::registered_part_route::RegisteredPartRoute;
use super::registered_part_route_map::RegisteredPartRouteMap;
use super::{
    ReadModel, ReadModelFragmentChangeEnvelope, ReadModelPartChange, ReadModelPartChangeError,
    ReadModelPartChangeRoute, ReadModelPartPathResolver, ReadModelPartTree,
};

/// Maps one source-fragment delivery stream through one read model's part tree.
pub struct ReadModelPartTreeMapper {
    routes: Vec<RegisteredPartRoute>,
}

impl ReadModelPartTreeMapper {
    /// Builds the normal and exceptional routes declared by one read model.
    pub fn for_read_model<R>() -> Self
    where
        R: ReadModel,
    {
        let parts = R::parts(None);
        let path_resolver = ReadModelPartPathResolver::new(parts);
        let mut routes = Vec::new();
        Self::register_explicit_routes(&mut routes, R::PART_CHANGE_ROUTES, path_resolver.clone());
        Self::register_parts(&mut routes, R::parts(None), path_resolver);
        Self { routes }
    }

    /// Converts a delivered physical fragment change into client-facing part changes.
    pub fn map(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
    ) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
        let mut mapped_changes = Vec::new();
        for change in &envelope.changes {
            for route in &self.routes {
                if route.fragment_name.value() != change.fragment_name().value() {
                    continue;
                }
                mapped_changes.extend(route.map.map(change, route.path_resolver.clone())?);
            }
        }
        Ok(mapped_changes)
    }

    fn register_explicit_routes(
        routes: &mut Vec<RegisteredPartRoute>,
        explicit_routes: &'static [ReadModelPartChangeRoute],
        path_resolver: ReadModelPartPathResolver,
    ) {
        for route in explicit_routes {
            routes.push(RegisteredPartRoute {
                fragment_name: route.fragment_name,
                map: RegisteredPartRouteMap::Explicit(route.map),
                path_resolver: path_resolver.clone(),
            });
        }
    }

    fn register_parts(
        routes: &mut Vec<RegisteredPartRoute>,
        parts: Vec<ReadModelPartTree>,
        path_resolver: ReadModelPartPathResolver,
    ) {
        for part in parts {
            if let Some(map) = part.map
                && !routes
                    .iter()
                    .any(|route| route.fragment_name == part.fragment_name)
            {
                routes.push(RegisteredPartRoute {
                    fragment_name: part.fragment_name,
                    map: RegisteredPartRouteMap::Tree(map),
                    path_resolver: path_resolver.clone(),
                });
            }
            Self::register_parts(routes, part.children, path_resolver.clone());
        }
    }
}
