use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Currency, CurrencyStatusManagerRelation};

/// Allows status managers to activate a currency.
pub struct CurrencyActivatorRelation;

impl Relation for CurrencyActivatorRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("activator"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: CurrencyStatusManagerRelation::REF,
    };
}
