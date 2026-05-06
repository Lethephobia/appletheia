use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationInvitation, OrganizationInvitationId, User, UserId,
};

use super::{
    OrganizationInvitationInviteeRelation, OrganizationInvitationOrganizationRelation,
    OrganizationInvitationRelationshipUpdater, OrganizationInvitationRelationshipUpdaterError,
};

pub struct DefaultOrganizationInvitationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultOrganizationInvitationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> OrganizationInvitationRelationshipUpdater
    for DefaultOrganizationInvitationRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_invitee(
        &self,
        uow: &mut Self::Uow,
        invitation_id: OrganizationInvitationId,
        invitee_id: UserId,
    ) -> Result<(), OrganizationInvitationRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    OrganizationInvitation,
                >(
                    invitation_id,
                    OrganizationInvitationInviteeRelation::REF,
                    RelationshipSubject::aggregate::<User>(invitee_id),
                ))],
            )
            .await?;

        Ok(())
    }

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        invitation_id: OrganizationInvitationId,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationInvitationRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    OrganizationInvitation,
                >(
                    invitation_id,
                    OrganizationInvitationOrganizationRelation::REF,
                    RelationshipSubject::aggregate::<Organization>(organization_id),
                ))],
            )
            .await?;

        Ok(())
    }
}
