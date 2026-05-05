use appletheia::application::authorization::{
    AggregateRef, Relation, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::{Account, AccountId, AccountOwner};

use super::{AccountOwnerRelation, AccountRelationshipUpdater, AccountRelationshipUpdaterError};

pub struct DefaultAccountRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultAccountRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn owner_subject(owner: AccountOwner) -> RelationshipSubject {
        match owner {
            AccountOwner::User(user_id) => RelationshipSubject::aggregate::<User>(user_id),
            AccountOwner::Organization(organization_id) => {
                RelationshipSubject::aggregate::<Organization>(organization_id)
            }
        }
    }
}

impl<RS> AccountRelationshipUpdater for DefaultAccountRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        account_id: AccountId,
        owner: AccountOwner,
    ) -> Result<(), AccountRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<Account>(
                    account_id,
                    AccountOwnerRelation::REF,
                    Self::owner_subject(owner),
                ))],
            )
            .await?;

        Ok(())
    }

    async fn replace_owner(
        &self,
        uow: &mut Self::Uow,
        account_id: AccountId,
        owner: AccountOwner,
    ) -> Result<(), AccountRelationshipUpdaterError> {
        let aggregate = AggregateRef::from_id::<Account>(account_id);
        let mut changes = self
            .relationship_store
            .read_subjects_by_aggregate(uow, &aggregate, &AccountOwnerRelation::REF.into(), None)
            .await?
            .into_iter()
            .map(|subject| {
                RelationshipChange::Delete(Relationship::new::<Account>(
                    account_id,
                    AccountOwnerRelation::REF,
                    subject,
                ))
            })
            .collect::<Vec<_>>();

        changes.push(RelationshipChange::Upsert(Relationship::new::<Account>(
            account_id,
            AccountOwnerRelation::REF,
            Self::owner_subject(owner),
        )));

        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }
}
