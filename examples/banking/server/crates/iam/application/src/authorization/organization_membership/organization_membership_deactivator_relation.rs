use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipStatusManagerRelation};

/// Allows status managers to deactivate a membership.
pub struct OrganizationMembershipDeactivatorRelation;

impl Relation for OrganizationMembershipDeactivatorRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationMembership::TYPE,
        RelationName::new("deactivator"),
    );

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationMembershipStatusManagerRelation::REF,
    };
}
