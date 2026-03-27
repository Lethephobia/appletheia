use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::UserOwnerRelation;

/// Defines the `profile_editor` relation for `User`.
pub struct UserProfileEditorRelation;

impl Relation for UserProfileEditorRelation {
    const NAME: RelationName = RelationName::new("profile_editor");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(UserOwnerRelation::NAME),
            },
        ])
    }
}
