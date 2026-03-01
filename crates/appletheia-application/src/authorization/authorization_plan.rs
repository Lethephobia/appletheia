use crate::projection::ProjectorDependencies;

use super::RelationshipRequirement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthorizationPlan {
    pub requirement: RelationshipRequirement,
    pub dependencies: ProjectorDependencies<'static>,
}

impl Default for AuthorizationPlan {
    fn default() -> Self {
        Self {
            requirement: RelationshipRequirement::None,
            dependencies: ProjectorDependencies::None,
        }
    }
}
