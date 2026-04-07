use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Allows direct organization tuples on currency definitions.
pub struct CurrencyDefinitionOrganizationRelation;

impl Relation for CurrencyDefinitionOrganizationRelation {
    const NAME: RelationName = RelationName::new("organization");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
