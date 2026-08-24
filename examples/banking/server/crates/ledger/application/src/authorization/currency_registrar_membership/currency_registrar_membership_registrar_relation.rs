use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembership;

/// Links a CurrencyRegistrarMembership to its registrar.
pub struct CurrencyRegistrarMembershipRegistrarRelation;

impl Relation for CurrencyRegistrarMembershipRegistrarRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarMembership::TYPE,
        RelationName::new("currency_registrar"),
    );
    const EXPR: UsersetExpr = UsersetExpr::This;
}
