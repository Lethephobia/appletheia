use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{User, UserId};
use banking_ledger_domain::currency_registrar::{CurrencyRegistrar, CurrencyRegistrarId};
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipId,
};

use super::{
    CurrencyRegistrarMemberRelation, CurrencyRegistrarMembershipRegistrarRelation,
    CurrencyRegistrarMembershipRelationshipUpdater,
    CurrencyRegistrarMembershipRelationshipUpdaterError,
};

pub struct DefaultCurrencyRegistrarMembershipRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultCurrencyRegistrarMembershipRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn registrar_relationship(
        currency_registrar_membership_id: CurrencyRegistrarMembershipId,
        currency_registrar_id: CurrencyRegistrarId,
    ) -> Relationship {
        Relationship::new::<CurrencyRegistrarMembership>(
            currency_registrar_membership_id,
            CurrencyRegistrarMembershipRegistrarRelation::REF,
            RelationshipSubject::aggregate::<CurrencyRegistrar>(currency_registrar_id),
        )
    }

    fn member_relationship(
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    ) -> Relationship {
        Relationship::new::<CurrencyRegistrar>(
            currency_registrar_id,
            CurrencyRegistrarMemberRelation::REF,
            RelationshipSubject::aggregate::<User>(user_id),
        )
    }
}

impl<RS> CurrencyRegistrarMembershipRelationshipUpdater
    for DefaultCurrencyRegistrarMembershipRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;

    async fn upsert_registrar(
        &self,
        uow: &mut Self::Uow,
        currency_registrar_membership_id: CurrencyRegistrarMembershipId,
        currency_registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRegistrarMembershipRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Self::registrar_relationship(
                    currency_registrar_membership_id,
                    currency_registrar_id,
                ))],
            )
            .await?;
        Ok(())
    }

    async fn upsert_member(
        &self,
        uow: &mut Self::Uow,
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    ) -> Result<(), CurrencyRegistrarMembershipRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Self::member_relationship(
                    currency_registrar_id,
                    user_id,
                ))],
            )
            .await?;
        Ok(())
    }

    async fn remove_member(
        &self,
        uow: &mut Self::Uow,
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    ) -> Result<(), CurrencyRegistrarMembershipRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Delete(Self::member_relationship(
                    currency_registrar_id,
                    user_id,
                ))],
            )
            .await?;
        Ok(())
    }
}
