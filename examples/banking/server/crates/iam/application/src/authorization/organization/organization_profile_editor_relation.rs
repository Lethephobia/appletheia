use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to edit organization profile attributes.
pub struct OrganizationProfileEditorRelation;

impl Relation for OrganizationProfileEditorRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("profile_editor"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: OrganizationAdminRelation::REF.into(),
        }
    }
}
