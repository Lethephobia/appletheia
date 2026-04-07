use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationJoinRequest, OrganizationJoinRequestEventPayload, User};

use super::{
    OrganizationJoinRequestRequesterRelationshipProjectorError,
    OrganizationJoinRequestRequesterRelationshipProjectorSpec,
};
use crate::authorization::OrganizationJoinRequestRequesterRelation;

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

            if let OrganizationJoinRequestEventPayload::Requested { requester_id, .. } =
                domain_event.payload()
            {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship {
                            aggregate,
                            relation: requester_relation,
                            subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(
                                *requester_id,
                            )),
                        })],
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
