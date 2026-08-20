use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to add a member directly.
///
/// This is the path an owner uses to give themselves an ordinary membership
/// before handing ownership over, which they must do while they still hold
/// administrative authority.
pub struct OrganizationMemberAdderRelation;

impl Relation for OrganizationMemberAdderRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("member_adder"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationAdminRelation::REF,
    };
}
