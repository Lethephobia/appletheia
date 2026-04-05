use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::{AggregateTypeOwned, EventEnvelope};
use appletheia::application::projection::Projector;
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationInvitation,
    OrganizationInvitationEventPayload, User,
};

use super::{
    OrganizationInvitationInviteeRelationshipProjectorError,
    OrganizationInvitationInviteeRelationshipProjectorSpec,
};
use crate::authorization::{
    OrganizationInvitationInviteeRelation, OrganizationInvitationOrganizationRelation,
};

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
                RelationNameOwned::from(OrganizationInvitationInviteeRelation::NAME);

            match domain_event.payload() {
                OrganizationInvitationEventPayload::Issued { invitee_id, .. } => {
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Upsert(Relationship {
                                aggregate,
                                relation: invitee_relation,
                                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<
                                    User,
                                >(
                                    *invitee_id
                                )),
                            })],
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::Accepted { invitee_id, .. }
                | OrganizationInvitationEventPayload::Declined { invitee_id, .. }
                | OrganizationInvitationEventPayload::Canceled { invitee_id, .. } => {
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Delete(Relationship {
                                aggregate,
                                relation: invitee_relation,
                                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<
                                    User,
                                >(
                                    *invitee_id
                                )),
                            })],
                        )
                        .await?;
                }
            }
        } else if aggregate_type == Organization::TYPE.value() {
            let domain_event = event.try_into_domain_event::<Organization>()?;

            if matches!(domain_event.payload(), OrganizationEventPayload::Removed) {
                let organization =
                    AggregateRef::from_id::<Organization>(domain_event.aggregate_id());
                let invitation_aggregate_type =
                    AggregateTypeOwned::from(OrganizationInvitation::TYPE);
                let organization_relation =
                    RelationNameOwned::from(OrganizationInvitationOrganizationRelation::NAME);
                let invitee_relation =
                    RelationNameOwned::from(OrganizationInvitationInviteeRelation::NAME);
                let invitation_aggregates = self
                    .relationship_store
                    .read_aggregates_by_subject(
                        uow,
                        &RelationshipSubject::Aggregate(organization.clone()),
                        &invitation_aggregate_type,
                        &organization_relation,
                    )
                    .await?;

                for invitation in invitation_aggregates {
                    let invitee_subjects = self
                        .relationship_store
                        .read_subjects_by_aggregate(uow, &invitation, &invitee_relation)
                        .await?;

                    if invitee_subjects.is_empty() {
                        continue;
                    }

                    let changes = invitee_subjects
                        .into_iter()
                        .map(|subject| {
                            RelationshipChange::Delete(Relationship {
                                aggregate: invitation.clone(),
                                relation: invitee_relation.clone(),
                                subject,
                            })
                        })
                        .collect::<Vec<_>>();

                    self.relationship_store.apply_changes(uow, &changes).await?;
                }
            }
        }

        Ok(())
    }
}
