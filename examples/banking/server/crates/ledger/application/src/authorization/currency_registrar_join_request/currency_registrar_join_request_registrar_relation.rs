use super::CurrencyRegistrarJoinRequestRegistrarDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_ledger_domain::{CurrencyRegistrar, CurrencyRegistrarJoinRequest};

/// Links a join request to its registrar.
pub struct CurrencyRegistrarJoinRequestRegistrarRelation;

impl Relation for CurrencyRegistrarJoinRequestRegistrarRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("registrar"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            CurrencyRegistrarJoinRequest,
            _,
            CurrencyRegistrarJoinRequestRegistrarDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<CurrencyRegistrarJoinRequest, CurrencyRegistrar>(
                aggregate.aggregate_id(),
                *aggregate.currency_registrar_id()?,
            );
            Ok(entries)
        })
    }
}
