use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::{AggregateTypeOwned, EventEnvelope};
use appletheia::application::projection::Projector;
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationJoinRequest,
    OrganizationJoinRequestEventPayload, User,
};

use super::{
    OrganizationJoinRequestRequesterRelationshipProjectorError,
    OrganizationJoinRequestRequesterRelationshipProjectorSpec,
};
use crate::authorization::{
    OrganizationJoinRequestOrganizationRelation, OrganizationJoinRequestRequesterRelation,
};

/// Projects the requester relationship for organization join requests.
pub struct OrganizationJoinRequestRequesterRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationJoinRequestRequesterRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for OrganizationJoinRequestRequesterRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationJoinRequestRequesterRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationJoinRequestRequesterRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let aggregate_type = event.aggregate_type.value();

        if aggregate_type == OrganizationJoinRequest::TYPE.value() {
            let domain_event = event.try_into_domain_event::<OrganizationJoinRequest>()?;
            let aggregate =
                AggregateRef::from_id::<OrganizationJoinRequest>(domain_event.aggregate_id());
            let requester_relation =
                RelationNameOwned::from(OrganizationJoinRequestRequesterRelation::NAME);

            match domain_event.payload() {
                OrganizationJoinRequestEventPayload::Requested { requester_id, .. } => {
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Upsert(Relationship {
                                aggregate,
                                relation: requester_relation,
                                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<
                                    User,
                                >(
                                    *requester_id
                                )),
                            })],
                        )
                        .await?;
                }
                OrganizationJoinRequestEventPayload::Approved { requester_id, .. }
                | OrganizationJoinRequestEventPayload::Rejected { requester_id, .. }
                | OrganizationJoinRequestEventPayload::Canceled { requester_id, .. } => {
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Delete(Relationship {
                                aggregate,
                                relation: requester_relation,
                                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<
                                    User,
                                >(
                                    *requester_id
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
                let request_aggregate_type =
                    AggregateTypeOwned::from(OrganizationJoinRequest::TYPE);
                let organization_relation =
                    RelationNameOwned::from(OrganizationJoinRequestOrganizationRelation::NAME);
                let requester_relation =
                    RelationNameOwned::from(OrganizationJoinRequestRequesterRelation::NAME);
                let request_aggregates = self
                    .relationship_store
                    .read_aggregates_by_subject(
                        uow,
                        &RelationshipSubject::Aggregate(organization.clone()),
                        &request_aggregate_type,
                        &organization_relation,
                    )
                    .await?;

                for request in request_aggregates {
                    let requester_subjects = self
                        .relationship_store
                        .read_subjects_by_aggregate(uow, &request, &requester_relation)
                        .await?;

                    if requester_subjects.is_empty() {
                        continue;
                    }

                    let changes = requester_subjects
                        .into_iter()
                        .map(|subject| {
                            RelationshipChange::Delete(Relationship {
                                aggregate: request.clone(),
                                relation: requester_relation.clone(),
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
