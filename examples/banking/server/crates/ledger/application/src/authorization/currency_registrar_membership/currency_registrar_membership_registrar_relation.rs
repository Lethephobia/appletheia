use super::CurrencyRegistrarMembershipRegistrarDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_ledger_domain::{CurrencyRegistrar, CurrencyRegistrarMembership};

/// Links a CurrencyRegistrarMembership to its registrar.
pub struct CurrencyRegistrarMembershipRegistrarRelation;

impl Relation for CurrencyRegistrarMembershipRegistrarRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarMembership::TYPE,
        RelationName::new("currency_registrar"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            CurrencyRegistrarMembership,
            _,
            CurrencyRegistrarMembershipRegistrarDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<CurrencyRegistrarMembership, CurrencyRegistrar>(
                aggregate.aggregate_id(),
                *aggregate.currency_registrar_id()?,
            );
            Ok(entries)
        })
    }
}
