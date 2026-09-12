use super::CurrencyRegistrarDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_ledger_domain::CurrencyRegistrar;
use banking_ledger_domain::currency::Currency;

/// Links a Currency to the registrar responsible for it.
pub struct CurrencyRegistrarRelation;

impl Relation for CurrencyRegistrarRelation {
    const REF: RelationRef =
        RelationRef::new(Currency::TYPE, RelationName::new("currency_registrar"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<Currency, _, CurrencyRegistrarDerivationHandlerError>(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<Currency, CurrencyRegistrar>(
                aggregate.aggregate_id(),
                *aggregate.currency_registrar_id()?,
            );
            Ok(entries)
        })
    }
}
