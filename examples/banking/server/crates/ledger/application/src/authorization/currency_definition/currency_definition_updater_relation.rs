use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::CurrencyDefinitionOwnerRelation;

/// Allows owners to update mutable currency-definition attributes.
pub struct CurrencyDefinitionUpdaterRelation;

impl Relation for CurrencyDefinitionUpdaterRelation {
    const NAME: RelationName = RelationName::new("updater");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This,
            UsersetExpr::ComputedUserset {
                relation: RelationNameOwned::from(CurrencyDefinitionOwnerRelation::NAME),
            },
        ])
    }
}
