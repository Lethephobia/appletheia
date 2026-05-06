use appletheia::application::authorization::{
    AggregateRef, Relation, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::currency::{Currency, CurrencyId, CurrencyOwner};

use super::{CurrencyOwnerRelation, CurrencyRelationshipUpdater, CurrencyRelationshipUpdaterError};

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

    fn owner_subject(owner: CurrencyOwner) -> RelationshipSubject {
        match owner {
            CurrencyOwner::User(user_id) => RelationshipSubject::aggregate::<User>(user_id),
            CurrencyOwner::Organization(organization_id) => {
                RelationshipSubject::aggregate::<Organization>(organization_id)
            }
        }
    }
}

impl<RS> CurrencyRelationshipUpdater for DefaultCurrencyRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        currency_id: CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<(), CurrencyRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<Currency>(
                    currency_id,
                    CurrencyOwnerRelation::REF,
                    Self::owner_subject(owner),
                ))],
            )
            .await?;

        Ok(())
    }

    async fn replace_owner(
        &self,
        uow: &mut Self::Uow,
        currency_id: CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<(), CurrencyRelationshipUpdaterError> {
        let aggregate = AggregateRef::from_id::<Currency>(currency_id);
        let mut changes = self
            .relationship_store
            .read_subjects_by_aggregate(uow, &aggregate, &CurrencyOwnerRelation::REF.into(), None)
            .await?
            .into_iter()
            .map(|subject| {
                RelationshipChange::Delete(Relationship::new::<Currency>(
                    currency_id,
                    CurrencyOwnerRelation::REF,
                    subject,
                ))
            })
            .collect::<Vec<_>>();

        changes.push(RelationshipChange::Upsert(Relationship::new::<Currency>(
            currency_id,
            CurrencyOwnerRelation::REF,
            Self::owner_subject(owner),
        )));

        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }
}
