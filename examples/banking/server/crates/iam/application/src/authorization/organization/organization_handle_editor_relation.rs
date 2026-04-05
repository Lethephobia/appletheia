use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationOwnerRelation;

/// Allows owners to edit an organization handle.
pub struct OrganizationHandleEditorRelation;

impl Relation for OrganizationHandleEditorRelation {
    const NAME: RelationName = RelationName::new("handle_editor");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
            },
        ])
    }
}
