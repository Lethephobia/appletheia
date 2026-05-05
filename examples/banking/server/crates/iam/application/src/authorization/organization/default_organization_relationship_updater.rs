use appletheia::application::authorization::{
    AggregateRef, Relation, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use banking_iam_domain::{Organization, OrganizationId, OrganizationOwner, User};

use super::{
    OrganizationOwnerRelation, OrganizationRelationshipUpdater,
    OrganizationRelationshipUpdaterError,
};

pub struct DefaultOrganizationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultOrganizationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn owner_subject(owner: OrganizationOwner) -> RelationshipSubject {
        match owner {
            OrganizationOwner::User(user_id) => RelationshipSubject::aggregate::<User>(user_id),
        }
    }
}

impl<RS> OrganizationRelationshipUpdater for DefaultOrganizationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        owner: OrganizationOwner,
    ) -> Result<(), OrganizationRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(
                    Relationship::new::<Organization>(
                        organization_id,
                        OrganizationOwnerRelation::REF,
                        Self::owner_subject(owner),
                    ),
                )],
            )
            .await?;

        Ok(())
    }

    async fn replace_owner(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        owner: OrganizationOwner,
    ) -> Result<(), OrganizationRelationshipUpdaterError> {
        let aggregate = AggregateRef::from_id::<Organization>(organization_id);
        let mut changes = self
            .relationship_store
            .read_subjects_by_aggregate(
                uow,
                &aggregate,
                &OrganizationOwnerRelation::REF.into(),
                None,
            )
            .await?
            .into_iter()
            .map(|subject| {
                RelationshipChange::Delete(Relationship::new::<Organization>(
                    organization_id,
                    OrganizationOwnerRelation::REF,
                    subject,
                ))
            })
            .collect::<Vec<_>>();

        changes.push(RelationshipChange::Upsert(
            Relationship::new::<Organization>(
                organization_id,
                OrganizationOwnerRelation::REF,
                Self::owner_subject(owner),
            ),
        ));

        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }
}
