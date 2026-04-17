use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Currency, CurrencyStatusManagerRelation};

/// Allows status managers to deactivate a currency.
pub struct CurrencyDeactivatorRelation;

impl Relation for CurrencyDeactivatorRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("deactivator"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyStatusManagerRelation::REF,
        },
    ]);
}
