use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Defines the `profile_editor` relation for `User`.
pub struct UserProfileEditorRelation;

impl Relation for UserProfileEditorRelation {
    const NAME: RelationName = RelationName::new("profile_editor");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
