use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_ledger_domain::currency::{Currency, CurrencyId};
use banking_ledger_domain::currency_registrar::{CurrencyRegistrar, CurrencyRegistrarId};

use super::{
    CurrencyRegistrarRelation, CurrencyRelationshipUpdater, CurrencyRelationshipUpdaterError,
};

pub struct DefaultCurrencyRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultCurrencyRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> CurrencyRelationshipUpdater for DefaultCurrencyRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;

    async fn upsert_currency_registrar(
        &self,
        uow: &mut Self::Uow,
        currency_id: CurrencyId,
        currency_registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<Currency>(
                    currency_id,
                    CurrencyRegistrarRelation::REF,
                    RelationshipSubject::aggregate::<CurrencyRegistrar>(currency_registrar_id),
                ))],
            )
            .await?;
        Ok(())
    }
}
