use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::CurrencyDefinitionStatusManagerRelation;

/// Allows status managers to remove a currency definition.
pub struct CurrencyDefinitionRemoverRelation;

impl Relation for CurrencyDefinitionRemoverRelation {
    const NAME: RelationName = RelationName::new("remover");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(CurrencyDefinitionStatusManagerRelation::NAME),
            },
        ])
    }
}
