use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::Currency;

/// Links a Currency to the registrar responsible for it.
pub struct CurrencyRegistrarRelation;

impl Relation for CurrencyRegistrarRelation {
    const REF: RelationRef =
        RelationRef::new(Currency::TYPE, RelationName::new("currency_registrar"));
    const EXPR: UsersetExpr = UsersetExpr::This;
}
