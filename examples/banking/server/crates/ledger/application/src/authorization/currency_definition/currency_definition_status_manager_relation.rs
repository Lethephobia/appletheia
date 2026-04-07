use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::CurrencyDefinitionOwnerRelation;

/// Allows owners to manage currency-definition status.
pub struct CurrencyDefinitionStatusManagerRelation;

impl Relation for CurrencyDefinitionStatusManagerRelation {
    const NAME: RelationName = RelationName::new("status_manager");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(CurrencyDefinitionOwnerRelation::NAME),
            },
        ])
    }
}
