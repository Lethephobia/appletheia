use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload, UserStatusManager};

use super::{
    UserStatusManagerRelationshipProjectorError, UserStatusManagerRelationshipProjectorSpec,
};
use crate::authorization::UserStatusManagerRelation;

/// Projects the status manager relationship for users.
pub struct UserStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> UserStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for UserStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = UserStatusManagerRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = UserStatusManagerRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<User>() {
            let event = event.try_into_domain_event::<User>()?;
            match event.payload() {
                UserEventPayload::StatusManagerAssigned { status_manager } => {
                    let UserStatusManager::User(status_manager) = *status_manager;
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Upsert(Relationship::new::<User>(
                                event.aggregate_id(),
                                UserStatusManagerRelation::REF,
                                RelationshipSubject::aggregate::<User>(status_manager),
                            ))],
                        )
                        .await?;
                }
                UserEventPayload::StatusManagerUnassigned { status_manager } => {
                    let UserStatusManager::User(status_manager) = *status_manager;
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Delete(Relationship::new::<User>(
                                event.aggregate_id(),
                                UserStatusManagerRelation::REF,
                                RelationshipSubject::aggregate::<User>(status_manager),
                            ))],
                        )
                        .await?;
                }
                _ => return Ok(()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, Relation, RelationRefOwned, RelationshipChange, RelationshipStore,
        RelationshipStoreError, RelationshipSubject,
    };
    use appletheia::application::event::{EventEnvelope, EventSequence, SerializedEventPayload};
    use appletheia::application::projection::Projector;
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};
    use banking_iam_domain::{
        Email, User, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject,
        UserStatusManager,
    };

    use super::UserStatusManagerRelationshipProjector;
    use crate::authorization::UserStatusManagerRelation;

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestRelationshipStore {
        changes: Arc<Mutex<Vec<RelationshipChange>>>,
    }

    impl TestRelationshipStore {
        fn recorded_changes(&self) -> Vec<RelationshipChange> {
            self.changes.lock().expect("lock should succeed").clone()
        }
    }

    impl RelationshipStore for TestRelationshipStore {
        type Uow = TestUow;

        async fn apply_changes(
            &self,
            _uow: &mut Self::Uow,
            changes: &[RelationshipChange],
        ) -> Result<(), RelationshipStoreError> {
            self.changes
                .lock()
                .expect("lock should succeed")
                .extend_from_slice(changes);
            Ok(())
        }

        async fn read_aggregates_by_subject(
            &self,
            _uow: &mut Self::Uow,
            _subject: &RelationshipSubject,
            _relation: &RelationRefOwned,
        ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
            Ok(Vec::new())
        }

        async fn read_subjects_by_aggregate(
            &self,
            _uow: &mut Self::Uow,
            _aggregate: &AggregateRef,
            _relation: &RelationRefOwned,
            _subject_aggregate_type: Option<&appletheia::application::event::AggregateTypeOwned>,
        ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
            Ok(Vec::new())
        }
    }

    fn identity() -> UserIdentity {
        UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            Some(Email::try_from("alice@example.com").expect("email should be valid")),
        )
    }

    fn status_manager_assigned_event_envelope() -> (EventEnvelope, UserStatusManager) {
        let status_manager_id = UserId::new();
        let status_manager = UserStatusManager::User(status_manager_id);
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        user.assign_status_manager(status_manager)
            .expect("status manager assignment should succeed");

        let event = user
            .uncommitted_events()
            .last()
            .expect("status manager event should exist")
            .clone();
        let message_id = MessageId::new();

        (
            EventEnvelope {
                event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
                event_id: event.id(),
                aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                    User::TYPE,
                ),
                aggregate_id: appletheia::application::event::AggregateIdValue::from(
                    event.aggregate_id().value(),
                ),
                aggregate_version: event.aggregate_version(),
                event_name: appletheia::application::event::EventNameOwned::from(
                    event.payload().name(),
                ),
                payload: SerializedEventPayload::try_from(
                    event
                        .payload()
                        .clone()
                        .into_json_value()
                        .expect("payload should serialize"),
                )
                .expect("payload should be valid"),
                occurred_at: event.occurred_at(),
                correlation_id: CorrelationId::from(message_id.value()),
                causation_id: CausationId::from(message_id),
                context: RequestContext::new(
                    CorrelationId::from(MessageId::new().value()),
                    message_id,
                    Principal::System,
                )
                .expect("request context should be valid"),
            },
            status_manager,
        )
    }

    fn status_manager_unassigned_event_envelope() -> (EventEnvelope, UserStatusManager) {
        let status_manager_id = UserId::new();
        let status_manager = UserStatusManager::User(status_manager_id);
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        user.unassign_status_manager(status_manager)
            .expect("status manager unassignment should succeed");

        let event = user
            .uncommitted_events()
            .last()
            .expect("status manager unassigned event should exist")
            .clone();
        let message_id = MessageId::new();

        (
            EventEnvelope {
                event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
                event_id: event.id(),
                aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                    User::TYPE,
                ),
                aggregate_id: appletheia::application::event::AggregateIdValue::from(
                    event.aggregate_id().value(),
                ),
                aggregate_version: event.aggregate_version(),
                event_name: appletheia::application::event::EventNameOwned::from(
                    event.payload().name(),
                ),
                payload: SerializedEventPayload::try_from(
                    event
                        .payload()
                        .clone()
                        .into_json_value()
                        .expect("payload should serialize"),
                )
                .expect("payload should be valid"),
                occurred_at: event.occurred_at(),
                correlation_id: CorrelationId::from(message_id.value()),
                causation_id: CausationId::from(message_id),
                context: RequestContext::new(
                    CorrelationId::from(MessageId::new().value()),
                    message_id,
                    Principal::System,
                )
                .expect("request context should be valid"),
            },
            status_manager,
        )
    }

    #[tokio::test]
    async fn project_status_manager_assigned_event_upserts_status_manager_relationship() {
        let store = TestRelationshipStore::default();
        let projector = UserStatusManagerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let (event, status_manager) = status_manager_assigned_event_envelope();

        projector
            .project(&mut uow, &event)
            .await
            .expect("projection should succeed");

        let changes = store.recorded_changes();
        assert_eq!(changes.len(), 1);

        let RelationshipChange::Upsert(relationship) = &changes[0] else {
            panic!("expected upsert relationship");
        };

        assert_eq!(
            relationship.relation,
            RelationRefOwned::from(UserStatusManagerRelation::REF)
        );
        let UserStatusManager::User(status_manager) = status_manager;
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(status_manager))
        );
    }

    #[tokio::test]
    async fn project_status_manager_unassigned_event_deletes_status_manager_relationship() {
        let store = TestRelationshipStore::default();
        let projector = UserStatusManagerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let (event, status_manager) = status_manager_unassigned_event_envelope();

        projector
            .project(&mut uow, &event)
            .await
            .expect("projection should succeed");

        let changes = store.recorded_changes();
        assert_eq!(changes.len(), 1);

        let RelationshipChange::Delete(relationship) = &changes[0] else {
            panic!("expected delete relationship");
        };

        assert_eq!(
            relationship.relation,
            RelationRefOwned::from(UserStatusManagerRelation::REF)
        );
        let UserStatusManager::User(status_manager) = status_manager;
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(status_manager))
        );
    }
}
