use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationInvitation, OrganizationInvitationEventPayload,
};

use super::{
    OrganizationInvitationOrganizationRelationshipProjectorError,
    OrganizationInvitationOrganizationRelationshipProjectorSpec,
};
use crate::authorization::OrganizationInvitationOrganizationRelation;

/// Projects the organization relationship for organization invitations.
pub struct OrganizationInvitationOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationInvitationOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for OrganizationInvitationOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationInvitationOrganizationRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationInvitationOrganizationRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<OrganizationInvitation>() {
            let domain_event = event.try_into_domain_event::<OrganizationInvitation>()?;

            if let OrganizationInvitationEventPayload::Issued {
                organization_id, ..
            } = domain_event.payload()
            {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship::new::<
                            OrganizationInvitation,
                        >(
                            domain_event.aggregate_id(),
                            OrganizationInvitationOrganizationRelation::REF,
                            RelationshipSubject::aggregate::<Organization>(*organization_id),
                        ))],
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
