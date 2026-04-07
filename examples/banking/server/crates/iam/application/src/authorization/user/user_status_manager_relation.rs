use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows directly assigned status managers to manage user status transitions.
pub struct UserStatusManagerRelation;

impl Relation for UserStatusManagerRelation {
    const NAME: RelationName = RelationName::new("status_manager");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
