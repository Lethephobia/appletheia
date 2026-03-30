use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows the owning subject itself.
pub struct CurrencyDefinitionOwnerRelation;

impl Relation for CurrencyDefinitionOwnerRelation {
    const NAME: RelationName = RelationName::new("owner");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
