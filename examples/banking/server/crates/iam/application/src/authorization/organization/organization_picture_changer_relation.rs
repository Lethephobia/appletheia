use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to change an organization picture.
pub struct OrganizationPictureChangerRelation;

impl Relation for OrganizationPictureChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("picture_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationAdminRelation::REF,
    };
}
