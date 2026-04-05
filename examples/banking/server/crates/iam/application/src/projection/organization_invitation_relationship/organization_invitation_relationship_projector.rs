use appletheia::application::authorization::Relation;
use appletheia::application::authorization::{
    AggregateRef, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationInvitation, OrganizationInvitationEventPayload, User,
};

use super::{
    OrganizationInvitationRelationshipProjectorError,
    OrganizationInvitationRelationshipProjectorSpec,
};
use crate::authorization::{
    OrganizationInvitationInviteeRelation, OrganizationInvitationOrganizationRelation,
};

/// Projects invitation relationships for authorization checks.
pub struct OrganizationInvitationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationInvitationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for OrganizationInvitationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationInvitationRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationInvitationRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<OrganizationInvitation>()?;

        match domain_event.payload() {
            OrganizationInvitationEventPayload::Issued {
                organization_id,
                invitee_id,
                ..
            } => {
                let aggregate =
                    AggregateRef::from_id::<OrganizationInvitation>(domain_event.aggregate_id());
                let organization_relation =
                    RelationNameOwned::from(OrganizationInvitationOrganizationRelation::NAME);
                let invitee_relation =
                    RelationNameOwned::from(OrganizationInvitationInviteeRelation::NAME);

                self.relationship_store
                    .apply_changes(
                        uow,
                        &[
                            RelationshipChange::Upsert(Relationship {
                                aggregate: aggregate.clone(),
                                relation: organization_relation,
                                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<
                                    Organization,
                                >(
                                    *organization_id
                                )),
                            }),
                            RelationshipChange::Upsert(Relationship {
                                aggregate,
                                relation: invitee_relation,
                                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<
                                    User,
                                >(
                                    *invitee_id
                                )),
                            }),
                        ],
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::Accepted { .. }
            | OrganizationInvitationEventPayload::Declined { .. }
            | OrganizationInvitationEventPayload::Canceled { .. } => {
                let aggregate =
                    AggregateRef::from_id::<OrganizationInvitation>(domain_event.aggregate_id());
                let organization_relation =
                    RelationNameOwned::from(OrganizationInvitationOrganizationRelation::NAME);
                let invitee_relation =
                    RelationNameOwned::from(OrganizationInvitationInviteeRelation::NAME);

                let organization_subjects = self
                    .relationship_store
                    .read_subjects_by_aggregate(uow, &aggregate, &organization_relation)
                    .await?;
                let invitee_subjects = self
                    .relationship_store
                    .read_subjects_by_aggregate(uow, &aggregate, &invitee_relation)
                    .await?;

                if organization_subjects.is_empty() && invitee_subjects.is_empty() {
                    return Ok(());
                }

                let organization_changes = organization_subjects.into_iter().map(|subject| {
                    RelationshipChange::Delete(Relationship {
                        aggregate: aggregate.clone(),
                        relation: organization_relation.clone(),
                        subject,
                    })
                });
                let invitee_changes = invitee_subjects.into_iter().map(|subject| {
                    RelationshipChange::Delete(Relationship {
                        aggregate: aggregate.clone(),
                        relation: invitee_relation.clone(),
                        subject,
                    })
                });

                self.relationship_store
                    .apply_changes(
                        uow,
                        &organization_changes
                            .chain(invitee_changes)
                            .collect::<Vec<_>>(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
