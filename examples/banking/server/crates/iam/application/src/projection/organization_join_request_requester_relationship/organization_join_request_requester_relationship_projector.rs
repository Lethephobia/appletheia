use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
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
        if event.is_for_aggregate::<OrganizationJoinRequest>() {
            let domain_event = event.try_into_domain_event::<OrganizationJoinRequest>()?;

            if let OrganizationJoinRequestEventPayload::Requested { requester_id, .. } =
                domain_event.payload()
            {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship::new::<
                            OrganizationJoinRequest,
                        >(
                            domain_event.aggregate_id(),
                            OrganizationJoinRequestRequesterRelation::REF,
                            RelationshipSubject::aggregate::<User>(*requester_id),
                        ))],
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
