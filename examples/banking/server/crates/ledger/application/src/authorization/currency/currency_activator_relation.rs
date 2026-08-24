use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::Currency;

use super::CurrencyManagerRelation;

pub struct CurrencyActivatorRelation;

impl Relation for CurrencyActivatorRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("activator"));
    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: CurrencyManagerRelation::REF,
    };
}
