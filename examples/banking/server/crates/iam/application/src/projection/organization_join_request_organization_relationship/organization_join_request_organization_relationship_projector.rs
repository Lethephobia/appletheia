use appletheia::application::authorization::{
    AggregateRef, Relation, RelationRefOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    Organization, OrganizationJoinRequest, OrganizationJoinRequestEventPayload,
};

use super::{
    OrganizationJoinRequestOrganizationRelationshipProjectorError,
    OrganizationJoinRequestOrganizationRelationshipProjectorSpec,
};
use crate::authorization::OrganizationJoinRequestOrganizationRelation;

/// Projects the organization relationship for organization join requests.
pub struct OrganizationJoinRequestOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationJoinRequestOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for OrganizationJoinRequestOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationJoinRequestOrganizationRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationJoinRequestOrganizationRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let aggregate_type = event.aggregate_type.value();

        if aggregate_type == OrganizationJoinRequest::TYPE.value() {
            let domain_event = event.try_into_domain_event::<OrganizationJoinRequest>()?;
            let aggregate =
                AggregateRef::from_id::<OrganizationJoinRequest>(domain_event.aggregate_id());
            let organization_relation =
                RelationRefOwned::from(OrganizationJoinRequestOrganizationRelation::REF);

            if let OrganizationJoinRequestEventPayload::Requested {
                organization_id, ..
            } = domain_event.payload()
            {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship {
                            aggregate,
                            relation: organization_relation,
                            subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<
                                Organization,
                            >(
                                *organization_id
                            )),
                        })],
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
