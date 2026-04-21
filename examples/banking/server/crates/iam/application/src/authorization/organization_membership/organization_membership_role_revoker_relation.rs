use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipRoleManagerRelation};

/// Allows role managers to revoke roles from a membership.
pub struct OrganizationMembershipRoleRevokerRelation;

impl Relation for OrganizationMembershipRoleRevokerRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationMembership::TYPE,
        RelationName::new("role_revoker"),
    );

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationMembershipRoleManagerRelation::REF,
    };
}
