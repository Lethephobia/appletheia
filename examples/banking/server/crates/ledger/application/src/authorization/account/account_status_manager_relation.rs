use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows owners to manage account status operations.
pub struct AccountStatusManagerRelation;

impl Relation for AccountStatusManagerRelation {
    const NAME: RelationName = RelationName::new("status_manager");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
