use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarId, CurrencyRegistrarInvitation,
    CurrencyRegistrarInvitationId, User, UserId,
};

use super::{
    CurrencyRegistrarInvitationInviteeRelation, CurrencyRegistrarInvitationRegistrarRelation,
    CurrencyRegistrarInvitationRelationshipUpdater,
    CurrencyRegistrarInvitationRelationshipUpdaterError,
};

pub struct DefaultCurrencyRegistrarInvitationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultCurrencyRegistrarInvitationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> CurrencyRegistrarInvitationRelationshipUpdater
    for DefaultCurrencyRegistrarInvitationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_invitee(
        &self,
        uow: &mut Self::Uow,
        invitation_id: CurrencyRegistrarInvitationId,
        invitee_id: UserId,
    ) -> Result<(), CurrencyRegistrarInvitationRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    CurrencyRegistrarInvitation,
                >(
                    invitation_id,
                    CurrencyRegistrarInvitationInviteeRelation::REF,
                    RelationshipSubject::aggregate::<User>(invitee_id),
                ))],
            )
            .await?;

        Ok(())
    }

    async fn upsert_registrar(
        &self,
        uow: &mut Self::Uow,
        invitation_id: CurrencyRegistrarInvitationId,
        registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRegistrarInvitationRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    CurrencyRegistrarInvitation,
                >(
                    invitation_id,
                    CurrencyRegistrarInvitationRegistrarRelation::REF,
                    RelationshipSubject::aggregate::<CurrencyRegistrar>(registrar_id),
                ))],
            )
            .await?;

        Ok(())
    }
}
