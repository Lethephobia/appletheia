use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyDefinition, CurrencyDefinitionStatusManagerRelation};

/// Allows status managers to remove a currency definition.
pub struct CurrencyDefinitionRemoverRelation;

impl Relation for CurrencyDefinitionRemoverRelation {
    const REF: RelationRef =
        RelationRef::new(CurrencyDefinition::TYPE, RelationName::new("remover"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyDefinitionStatusManagerRelation::REF,
        },
    ]);
}
