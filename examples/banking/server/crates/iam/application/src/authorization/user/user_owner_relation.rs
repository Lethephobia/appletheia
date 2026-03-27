use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows the owning user itself.
pub struct UserOwnerRelation;

impl Relation for UserOwnerRelation {
    const NAME: RelationName = RelationName::new("owner");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
