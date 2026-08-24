use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_registrar::CurrencyRegistrar;

/// Allows active members to operate a CurrencyRegistrar.
pub struct CurrencyRegistrarMemberRelation;

impl Relation for CurrencyRegistrarMemberRelation {
    const REF: RelationRef = RelationRef::new(CurrencyRegistrar::TYPE, RelationName::new("member"));
    const EXPR: UsersetExpr = UsersetExpr::This;
}
