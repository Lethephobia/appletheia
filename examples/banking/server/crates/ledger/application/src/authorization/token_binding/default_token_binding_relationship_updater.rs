use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_ledger_domain::currency::{Currency, CurrencyId};
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingId};

use super::{
    TokenBindingCurrencyRelation, TokenBindingRelationshipUpdater,
    TokenBindingRelationshipUpdaterError,
};

pub struct DefaultTokenBindingRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultTokenBindingRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> TokenBindingRelationshipUpdater for DefaultTokenBindingRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        token_binding_id: TokenBindingId,
        currency_id: CurrencyId,
    ) -> Result<(), TokenBindingRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(
                    Relationship::new::<TokenBinding>(
                        token_binding_id,
                        TokenBindingCurrencyRelation::REF,
                        RelationshipSubject::aggregate::<Currency>(currency_id),
                    ),
                )],
            )
            .await?;
        Ok(())
    }
}
