use super::{
    CurrencyRegistrarJoinRequestRegistrarRelation, CurrencyRegistrarJoinRequestRelationshipUpdater,
    CurrencyRegistrarJoinRequestRelationshipUpdaterError,
    CurrencyRegistrarJoinRequestRequesterRelation,
};
use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarId, CurrencyRegistrarJoinRequest,
    CurrencyRegistrarJoinRequestId, User, UserId,
};

pub struct DefaultCurrencyRegistrarJoinRequestRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultCurrencyRegistrarJoinRequestRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> CurrencyRegistrarJoinRequestRelationshipUpdater
    for DefaultCurrencyRegistrarJoinRequestRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_registrar(
        &self,
        uow: &mut Self::Uow,
        join_request_id: CurrencyRegistrarJoinRequestId,
        registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRegistrarJoinRequestRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    CurrencyRegistrarJoinRequest,
                >(
                    join_request_id,
                    CurrencyRegistrarJoinRequestRegistrarRelation::REF,
                    RelationshipSubject::aggregate::<CurrencyRegistrar>(registrar_id),
                ))],
            )
            .await?;

        Ok(())
    }

    async fn upsert_requester(
        &self,
        uow: &mut Self::Uow,
        join_request_id: CurrencyRegistrarJoinRequestId,
        requester_id: UserId,
    ) -> Result<(), CurrencyRegistrarJoinRequestRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    CurrencyRegistrarJoinRequest,
                >(
                    join_request_id,
                    CurrencyRegistrarJoinRequestRequesterRelation::REF,
                    RelationshipSubject::aggregate::<User>(requester_id),
                ))],
            )
            .await?;

        Ok(())
    }
}
