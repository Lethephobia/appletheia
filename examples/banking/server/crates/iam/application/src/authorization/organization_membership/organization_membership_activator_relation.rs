use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipStatusManagerRelation};

/// Allows status managers to activate a membership.
pub struct OrganizationMembershipActivatorRelation;

impl Relation for OrganizationMembershipActivatorRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationMembership::TYPE, RelationName::new("activator"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationMembershipStatusManagerRelation::REF,
        },
    ]);
}
