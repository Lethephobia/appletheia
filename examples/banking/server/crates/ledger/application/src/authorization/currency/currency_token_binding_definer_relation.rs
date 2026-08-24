use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::Currency;

use super::CurrencyManagerRelation;

pub struct CurrencyTokenBindingDefinerRelation;

impl Relation for CurrencyTokenBindingDefinerRelation {
    const REF: RelationRef =
        RelationRef::new(Currency::TYPE, RelationName::new("token_binding_definer"));
    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: CurrencyManagerRelation::REF,
    };
}
