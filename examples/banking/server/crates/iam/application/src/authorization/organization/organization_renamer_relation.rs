use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationOwnerRelation;

/// Allows owners to rename an organization.
pub struct OrganizationRenamerRelation;

impl Relation for OrganizationRenamerRelation {
    const NAME: RelationName = RelationName::new("renamer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
            },
        ])
    }
}
