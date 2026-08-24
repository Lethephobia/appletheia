use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_registrar::CurrencyRegistrar;

use crate::authorization::CurrencyRegistrarMemberRelation;

pub struct CurrencyRegistrarCurrencyDefinerRelation;

impl Relation for CurrencyRegistrarCurrencyDefinerRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrar::TYPE,
        RelationName::new("currency_definer"),
    );
    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: CurrencyRegistrarMemberRelation::REF,
    };
}
