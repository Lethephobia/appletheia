use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows direct tuples to manage status transitions.
pub struct CurrencyDefinitionStatusManagerRelation;

impl Relation for CurrencyDefinitionStatusManagerRelation {
    const NAME: RelationName = RelationName::new("status_manager");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
