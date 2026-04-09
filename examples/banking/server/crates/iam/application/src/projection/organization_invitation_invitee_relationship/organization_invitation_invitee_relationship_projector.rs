use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{OrganizationInvitation, OrganizationInvitationEventPayload, User};

use super::{
    OrganizationInvitationInviteeRelationshipProjectorError,
    OrganizationInvitationInviteeRelationshipProjectorSpec,
};
use crate::authorization::OrganizationInvitationInviteeRelation;

/// Projects the invitee relationship for organization invitations.
pub struct OrganizationInvitationInviteeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationInvitationInviteeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for OrganizationInvitationInviteeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationInvitationInviteeRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationInvitationInviteeRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<OrganizationInvitation>() {
            let domain_event = event.try_into_domain_event::<OrganizationInvitation>()?;

            if let OrganizationInvitationEventPayload::Issued { invitee_id, .. } =
                domain_event.payload()
            {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship::new::<
                            OrganizationInvitation,
                        >(
                            domain_event.aggregate_id(),
                            OrganizationInvitationInviteeRelation::REF,
                            RelationshipSubject::aggregate::<User>(*invitee_id),
                        ))],
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
