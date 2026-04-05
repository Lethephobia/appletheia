use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationOwnerRelation;

/// Allows owners to invite users to an organization.
pub struct OrganizationInviterRelation;

impl Relation for OrganizationInviterRelation {
    const NAME: RelationName = RelationName::new("inviter");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
            },
        ])
    }
}
