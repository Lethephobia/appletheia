use super::CurrencyRegistrarMemberDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::User;
use banking_ledger_domain::{CurrencyRegistrar, CurrencyRegistrarMembership};

/// Allows active members to operate a CurrencyRegistrar.
pub struct CurrencyRegistrarMemberRelation;

impl Relation for CurrencyRegistrarMemberRelation {
    const REF: RelationRef = RelationRef::new(CurrencyRegistrar::TYPE, RelationName::new("member"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            CurrencyRegistrarMembership,
            _,
            CurrencyRegistrarMemberDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            if aggregate.is_active()? {
                entries.insert::<CurrencyRegistrar, User>(
                    *aggregate.currency_registrar_id()?,
                    *aggregate.user_id()?,
                );
            }
            Ok(entries)
        })
    }
}
