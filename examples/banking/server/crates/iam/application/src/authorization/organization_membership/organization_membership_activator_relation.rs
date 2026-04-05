use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationMembershipStatusManagerRelation;

/// Allows status managers to activate a membership.
pub struct OrganizationMembershipActivatorRelation;

impl Relation for OrganizationMembershipActivatorRelation {
    const NAME: RelationName = RelationName::new("activator");

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
