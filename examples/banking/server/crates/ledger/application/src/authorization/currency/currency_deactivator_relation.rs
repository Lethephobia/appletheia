use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::Currency;

use super::CurrencyManagerRelation;

pub struct CurrencyDeactivatorRelation;

impl Relation for CurrencyDeactivatorRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("deactivator"));
    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: CurrencyManagerRelation::REF,
    };
}
