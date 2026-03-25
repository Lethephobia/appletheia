use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::{
    AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventSelector,
};
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::ProjectorDefinition;
use appletheia::domain::{Aggregate, AggregateId};
use banking_iam_domain::{
    Role, RoleId, User, UserId, UserRoleAssignment, UserRoleAssignmentEventPayload,
};

use super::{RoleAssigneeRelationshipProjector, RoleAssigneeRelationshipProjectorError};
use crate::authorization::RoleAssigneeRelation;

/// Projects `role#assignee@user` relationships from `UserRoleAssignment` events.
pub struct DefaultRoleAssigneeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultRoleAssigneeRelationshipProjector<RS>
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

impl<RS> ProjectorDefinition for DefaultRoleAssigneeRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    type Error = RoleAssigneeRelationshipProjectorError;

    const NAME: appletheia::application::projection::ProjectorName =
        RoleAssigneeRelationshipProjector::NAME;
    const SUBSCRIPTION: Subscription<'static, EventSelector> = Subscription::Only(&[
        EventSelector::new(
            UserRoleAssignment::TYPE,
            UserRoleAssignmentEventPayload::ASSIGNED,
        ),
        EventSelector::new(
            UserRoleAssignment::TYPE,
            UserRoleAssignmentEventPayload::REVOKED,
        ),
    ]);

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
