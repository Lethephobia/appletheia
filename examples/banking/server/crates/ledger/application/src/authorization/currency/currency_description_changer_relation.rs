use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::Currency;

use super::CurrencyManagerRelation;

pub struct CurrencyDescriptionChangerRelation;

impl Relation for CurrencyDescriptionChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Currency::TYPE, RelationName::new("description_changer"));
    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: CurrencyManagerRelation::REF,
    };
}
