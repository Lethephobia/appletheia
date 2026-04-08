use appletheia::application::authorization::{
    AggregateRef, Relation, RelationRefOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::domain::Aggregate;
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
        let aggregate_type = event.aggregate_type.value();

        if aggregate_type == OrganizationInvitation::TYPE.value() {
            let domain_event = event.try_into_domain_event::<OrganizationInvitation>()?;
            let aggregate =
                AggregateRef::from_id::<OrganizationInvitation>(domain_event.aggregate_id());
            let invitee_relation =
                RelationRefOwned::from(OrganizationInvitationInviteeRelation::REF);

            if let OrganizationInvitationEventPayload::Issued { invitee_id, .. } =
                domain_event.payload()
            {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship {
                            aggregate,
                            relation: invitee_relation,
                            subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(
                                *invitee_id,
                            )),
                        })],
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
