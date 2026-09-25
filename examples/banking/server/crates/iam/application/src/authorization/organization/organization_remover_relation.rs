use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to remove an organization.
pub struct OrganizationRemoverRelation;

impl Relation for OrganizationRemoverRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("remover"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: OrganizationAdminRelation::REF.into(),
        }
    }
}
