use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Currency, CurrencyStatusManagerRelation};

/// Allows status managers to remove a currency.
pub struct CurrencyRemoverRelation;

impl Relation for CurrencyRemoverRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("remover"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyStatusManagerRelation::REF,
        },
    ]);
}
