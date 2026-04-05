use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationOwnerRelation;

/// Allows direct members and the owning subject to count as organization members.
pub struct OrganizationMemberRelation;

impl Relation for OrganizationMemberRelation {
    const NAME: RelationName = RelationName::new("member");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
            },
        ])
    }
}
