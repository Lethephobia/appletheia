use crate::projection::ProjectorDependencies;

use super::RelationshipRequirement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrincipalRequirement {
    System,
    Anonymous,
    Authenticated,
    AuthenticatedWithRelationship {
        requirement: RelationshipRequirement,
        projector_dependencies: ProjectorDependencies<'static>,
    },
}
