use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationOwnerRelation;

/// Allows owners to remove an organization.
pub struct OrganizationRemoverRelation;

impl Relation for OrganizationRemoverRelation {
    const NAME: RelationName = RelationName::new("remover");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
            },
        ])
    }
}
