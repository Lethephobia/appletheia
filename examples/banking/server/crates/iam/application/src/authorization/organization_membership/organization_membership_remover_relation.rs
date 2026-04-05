use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationMembershipStatusManagerRelation;

/// Allows status managers to remove a membership.
pub struct OrganizationMembershipRemoverRelation;

impl Relation for OrganizationMembershipRemoverRelation {
    const NAME: RelationName = RelationName::new("remover");

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
