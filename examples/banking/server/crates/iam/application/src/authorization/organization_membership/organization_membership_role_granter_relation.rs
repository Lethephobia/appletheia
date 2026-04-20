use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipRoleManagerRelation};

/// Allows role managers to grant roles to a membership.
pub struct OrganizationMembershipRoleGranterRelation;

impl Relation for OrganizationMembershipRoleGranterRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationMembership::TYPE,
        RelationName::new("role_granter"),
    );

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationMembershipRoleManagerRelation::REF,
        },
    ]);
}
