use crate::projection::ProjectorDependencies;

use super::RelationshipRequirement;

/// Describes which kind of principal may satisfy an authorization check.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrincipalRequirement {
    /// Requires the system principal.
    System,
    /// Allows an anonymous principal.
    Anonymous,
    /// Requires any authenticated principal.
    Authenticated,
    /// Requires an authenticated principal that also satisfies a relationship check.
    AuthenticatedWithRelationship {
        /// The relationship the principal must satisfy.
        requirement: RelationshipRequirement,
        /// Projectors that must be up to date before the relationship is evaluated.
        projector_dependencies: ProjectorDependencies<'static>,
    },
}
