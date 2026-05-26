use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::payout_destination::{
    PayoutDestination, PayoutDestinationId, PayoutDestinationOwner,
};

use super::{
    PayoutDestinationOwnerRelation, PayoutDestinationRelationshipUpdater,
    PayoutDestinationRelationshipUpdaterError,
};

pub struct DefaultPayoutDestinationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultPayoutDestinationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn owner_subject(owner: PayoutDestinationOwner) -> RelationshipSubject {
        match owner {
            PayoutDestinationOwner::User(user_id) => {
                RelationshipSubject::aggregate::<User>(user_id)
            }
            PayoutDestinationOwner::Organization(organization_id) => {
                RelationshipSubject::aggregate::<Organization>(organization_id)
            }
        }
    }
}

impl<RS> PayoutDestinationRelationshipUpdater for DefaultPayoutDestinationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        payout_destination_id: PayoutDestinationId,
        owner: PayoutDestinationOwner,
    ) -> Result<(), PayoutDestinationRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    PayoutDestination,
                >(
                    payout_destination_id,
                    PayoutDestinationOwnerRelation::REF,
                    Self::owner_subject(owner),
                ))],
            )
            .await?;

        Ok(())
    }
}
