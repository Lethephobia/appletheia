use super::{
    OrganizationJoinRequestOrganizationRelation, OrganizationJoinRequestRelationshipUpdater,
    OrganizationJoinRequestRelationshipUpdaterError, OrganizationJoinRequestRequesterRelation,
};
use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationJoinRequest, OrganizationJoinRequestId, User, UserId,
};

pub struct DefaultOrganizationJoinRequestRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultOrganizationJoinRequestRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> OrganizationJoinRequestRelationshipUpdater
    for DefaultOrganizationJoinRequestRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        join_request_id: OrganizationJoinRequestId,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationJoinRequestRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    OrganizationJoinRequest,
                >(
                    join_request_id,
                    OrganizationJoinRequestOrganizationRelation::REF,
                    RelationshipSubject::aggregate::<Organization>(organization_id),
                ))],
            )
            .await?;

        Ok(())
    }

    async fn upsert_requester(
        &self,
        uow: &mut Self::Uow,
        join_request_id: OrganizationJoinRequestId,
        requester_id: UserId,
    ) -> Result<(), OrganizationJoinRequestRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    OrganizationJoinRequest,
                >(
                    join_request_id,
                    OrganizationJoinRequestRequesterRelation::REF,
                    RelationshipSubject::aggregate::<User>(requester_id),
                ))],
            )
            .await?;

        Ok(())
    }
}
