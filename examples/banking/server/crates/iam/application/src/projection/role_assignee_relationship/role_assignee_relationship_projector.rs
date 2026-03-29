use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Role, User, UserRoleAssignment, UserRoleAssignmentEventPayload};

use super::{RoleAssigneeRelationshipProjectorError, RoleAssigneeRelationshipProjectorSpec};
use crate::authorization::RoleAssigneeRelation;

/// Projects `role#assignee@user` relationships from `UserRoleAssignment` events.
pub struct RoleAssigneeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> RoleAssigneeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for RoleAssigneeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = RoleAssigneeRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = RoleAssigneeRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event = event.try_into_domain_event::<UserRoleAssignment>()?;

        let change = match event.payload() {
            UserRoleAssignmentEventPayload::Assigned {
                role_id, user_id, ..
            } => RelationshipChange::Upsert(Relationship {
                aggregate: AggregateRef::from_id::<Role>(*role_id),
                relation: RelationNameOwned::from(RoleAssigneeRelation::NAME),
                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(*user_id)),
            }),
            UserRoleAssignmentEventPayload::Revoked {
                role_id, user_id, ..
            } => RelationshipChange::Delete(Relationship {
                aggregate: AggregateRef::from_id::<Role>(*role_id),
                relation: RelationNameOwned::from(RoleAssigneeRelation::NAME),
                subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(*user_id)),
            }),
        };

        self.relationship_store
            .apply_changes(uow, &[change])
            .await?;
        Ok(())
    }
}
