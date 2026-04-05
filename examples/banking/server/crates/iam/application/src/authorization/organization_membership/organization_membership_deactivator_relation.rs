use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationMembershipStatusManagerRelation;

/// Allows status managers to deactivate a membership.
pub struct OrganizationMembershipDeactivatorRelation;

impl Relation for OrganizationMembershipDeactivatorRelation {
    const NAME: RelationName = RelationName::new("deactivator");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(
                    OrganizationMembershipStatusManagerRelation::NAME,
                ),
            },
        ])
    }
}
