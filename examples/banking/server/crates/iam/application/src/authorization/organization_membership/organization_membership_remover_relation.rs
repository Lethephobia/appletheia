use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipStatusManagerRelation};

/// Allows status managers to remove a membership.
pub struct OrganizationMembershipRemoverRelation;

impl Relation for OrganizationMembershipRemoverRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationMembership::TYPE, RelationName::new("remover"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationMembershipStatusManagerRelation::REF,
        },
    ]);
}
