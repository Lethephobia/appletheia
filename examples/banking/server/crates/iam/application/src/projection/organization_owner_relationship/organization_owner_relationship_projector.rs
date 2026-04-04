use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::request_context::ActorRef;
use banking_iam_domain::Organization;

use super::{
    OrganizationOwnerRelationshipProjectorError, OrganizationOwnerRelationshipProjectorSpec,
};
use crate::authorization::OrganizationOwnerRelation;
use banking_iam_domain::OrganizationEventPayload;

/// Projects the initial owner relationship for new organizations.
pub struct OrganizationOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for OrganizationOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationOwnerRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationOwnerRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<Organization>()?;

        match domain_event.payload() {
            OrganizationEventPayload::Created { .. } => {
                let ActorRef::Subject { subject } = &event.context.actor else {
                    return Ok(());
                };

                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship {
                            aggregate: AggregateRef::from_id::<Organization>(
                                domain_event.aggregate_id(),
                            ),
                            relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
                            subject: RelationshipSubject::Aggregate(subject.clone()),
                        })],
                    )
                    .await?;
            }
            OrganizationEventPayload::Removed => {
                let aggregate = AggregateRef::from_id::<Organization>(domain_event.aggregate_id());
                let relation = RelationNameOwned::from(OrganizationOwnerRelation::NAME);
                let subjects = self
                    .relationship_store
                    .read_subjects_by_aggregate(uow, &aggregate, &relation)
                    .await?;

                if subjects.is_empty() {
                    return Ok(());
                }

                let changes = subjects
                    .into_iter()
                    .map(|subject| {
                        RelationshipChange::Delete(Relationship {
                            aggregate: aggregate.clone(),
                            relation: relation.clone(),
                            subject,
                        })
                    })
                    .collect::<Vec<_>>();

                self.relationship_store.apply_changes(uow, &changes).await?;
            }
            OrganizationEventPayload::HandleChanged { .. } => return Ok(()),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, Relation, RelationNameOwned, RelationshipChange, RelationshipStore,
        RelationshipStoreError, RelationshipSubject,
    };
    use appletheia::application::event::{EventEnvelope, EventSequence, SerializedEventPayload};
    use appletheia::application::projection::Projector;
    use appletheia::application::request_context::{
        ActorRef, CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};
    use banking_iam_domain::{Organization, OrganizationHandle, OrganizationName, User, UserId};

    use super::OrganizationOwnerRelationshipProjector;
    use crate::authorization::OrganizationOwnerRelation;

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
        subjects_by_aggregate: Arc<Mutex<Vec<RelationshipSubject>>>,
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
            _aggregate_type: &appletheia::application::event::AggregateTypeOwned,
            _relation: &RelationNameOwned,
        ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
            Ok(Vec::new())
        }

        async fn read_subjects_by_aggregate(
            &self,
            _uow: &mut Self::Uow,
            _aggregate: &AggregateRef,
            _relation: &RelationNameOwned,
        ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
            Ok(self
                .subjects_by_aggregate
                .lock()
                .expect("lock should succeed")
                .clone())
        }
    }

    fn created_event_envelope() -> (EventEnvelope, AggregateRef) {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("creation should succeed");

        let event = organization
            .uncommitted_events()
            .first()
            .expect("created event should exist")
            .clone();
        let subject = AggregateRef::from_id::<User>(UserId::new());
        let actor_subject = subject.clone();
        let message_id = MessageId::new();

        (
            EventEnvelope {
                event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
                event_id: event.id(),
                aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                    Organization::TYPE,
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
                    ActorRef::Subject {
                        subject: actor_subject.clone(),
                    },
                    Principal::Authenticated {
                        subject: actor_subject,
                    },
                ),
            },
            subject,
        )
    }

    fn removed_event_envelope() -> EventEnvelope {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("creation should succeed");
        organization.remove().expect("remove should succeed");

        let event = organization
            .uncommitted_events()
            .last()
            .expect("removed event should exist")
            .clone();
        let subject = AggregateRef::from_id::<User>(UserId::new());
        let message_id = MessageId::new();

        EventEnvelope {
            event_sequence: EventSequence::try_from(2).expect("sequence should be valid"),
            event_id: event.id(),
            aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                Organization::TYPE,
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
                MessageId::new(),
                ActorRef::Subject {
                    subject: subject.clone(),
                },
                Principal::Authenticated { subject },
            ),
        }
    }

    #[tokio::test]
    async fn project_created_event_upserts_owner_relationship() {
        let store = TestRelationshipStore::default();
        let projector = OrganizationOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let (event, subject) = created_event_envelope();

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
            RelationNameOwned::from(OrganizationOwnerRelation::NAME)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(subject)
        );
    }

    #[tokio::test]
    async fn project_removed_event_deletes_owner_relationship() {
        let store = TestRelationshipStore {
            subjects_by_aggregate: Arc::new(Mutex::new(vec![RelationshipSubject::Aggregate(
                AggregateRef::from_id::<User>(UserId::new()),
            )])),
            ..Default::default()
        };
        let projector = OrganizationOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let event = removed_event_envelope();

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
            RelationNameOwned::from(OrganizationOwnerRelation::NAME)
        );
    }
}
