use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::CurrencyDefinitionStatusManagerRelation;

/// Allows status managers to deactivate a currency definition.
pub struct CurrencyDefinitionDeactivatorRelation;

impl Relation for CurrencyDefinitionDeactivatorRelation {
    const NAME: RelationName = RelationName::new("deactivator");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(CurrencyDefinitionStatusManagerRelation::NAME),
            },
        ])
    }
}
