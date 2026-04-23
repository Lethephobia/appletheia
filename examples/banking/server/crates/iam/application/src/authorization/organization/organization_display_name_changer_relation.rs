use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to change an organization display name.
pub struct OrganizationDisplayNameChangerRelation;

impl Relation for OrganizationDisplayNameChangerRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("display_name_changer"),
    );

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationAdminRelation::REF,
    };
}
