use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::CurrencyDefinitionStatusManagerRelation;

/// Allows status managers to activate a currency definition.
pub struct CurrencyDefinitionActivatorRelation;

impl Relation for CurrencyDefinitionActivatorRelation {
    const NAME: RelationName = RelationName::new("activator");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(CurrencyDefinitionStatusManagerRelation::NAME),
            },
        ])
    }
}
