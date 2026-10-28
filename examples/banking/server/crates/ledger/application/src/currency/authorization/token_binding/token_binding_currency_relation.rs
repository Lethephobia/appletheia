use super::TokenBindingCurrencyDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::token_binding::TokenBinding;

pub struct TokenBindingCurrencyRelation;

impl Relation for TokenBindingCurrencyRelation {
    const REF: RelationRef = RelationRef::new(TokenBinding::TYPE, RelationName::new("currency"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<TokenBinding, _, TokenBindingCurrencyDerivationHandlerError>(
            |aggregate| {
                let mut entries = RelationshipEntries::new();
                entries.insert::<TokenBinding, Currency>(
                    aggregate.aggregate_id(),
                    aggregate.currency_id()?,
                );
                Ok(entries)
            },
        )
    }
}
