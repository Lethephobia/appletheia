use super::CurrencyRegistrarInvitationRegistrarDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_ledger_domain::{CurrencyRegistrar, CurrencyRegistrarInvitation};

/// Links an invitation to its registrar.
pub struct CurrencyRegistrarInvitationRegistrarRelation;

impl Relation for CurrencyRegistrarInvitationRegistrarRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarInvitation::TYPE,
        RelationName::new("registrar"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            CurrencyRegistrarInvitation,
            _,
            CurrencyRegistrarInvitationRegistrarDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<CurrencyRegistrarInvitation, CurrencyRegistrar>(
                aggregate.aggregate_id(),
                *aggregate.currency_registrar_id()?,
            );
            Ok(entries)
        })
    }
}
