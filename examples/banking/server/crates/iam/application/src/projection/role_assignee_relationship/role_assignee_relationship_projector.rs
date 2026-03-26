use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::{AggregateIdValue, AggregateTypeOwned, EventEnvelope};
use appletheia::application::projection::Projector;
use appletheia::domain::{Aggregate, AggregateId};
use banking_iam_domain::{
    Role, RoleId, User, UserId, UserRoleAssignment, UserRoleAssignmentEventPayload,
};

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

    fn relationship(role_id: RoleId, user_id: UserId) -> Relationship {
        Relationship {
            aggregate: AggregateRef {
                aggregate_type: AggregateTypeOwned::from(Role::TYPE),
                aggregate_id: AggregateIdValue::from(role_id.value()),
            },
            relation: RelationNameOwned::from(RoleAssigneeRelation::NAME),
            subject: RelationshipSubject::Aggregate(AggregateRef {
                aggregate_type: AggregateTypeOwned::from(User::TYPE),
                aggregate_id: AggregateIdValue::from(user_id.value()),
            }),
        }
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
            } => RelationshipChange::Upsert(Self::relationship(*role_id, *user_id)),
            UserRoleAssignmentEventPayload::Revoked {
                role_id, user_id, ..
            } => RelationshipChange::Delete(Self::relationship(*role_id, *user_id)),
        };

        self.relationship_store
            .apply_changes(uow, &[change])
            .await?;
        Ok(())
    }
}
