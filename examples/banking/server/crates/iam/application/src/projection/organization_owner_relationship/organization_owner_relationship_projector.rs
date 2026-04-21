use appletheia::application::authorization::{
    AggregateRef, Relation, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::Organization;

use super::{
    OrganizationOwnerRelationshipProjectorError, OrganizationOwnerRelationshipProjectorSpec,
};
use crate::authorization::OrganizationOwnerRelation;
use banking_iam_domain::{OrganizationEventPayload, OrganizationOwner, User};

/// Projects the owner relationship for organizations.
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

    fn owner_subject(owner: OrganizationOwner) -> RelationshipSubject {
        let OrganizationOwner::User(owner) = owner;
        RelationshipSubject::aggregate::<User>(owner)
    }

    async fn replace_owner_relationships(
        &self,
        uow: &mut RS::Uow,
        organization_id: banking_iam_domain::OrganizationId,
        owner: OrganizationOwner,
    ) -> Result<(), OrganizationOwnerRelationshipProjectorError> {
        let aggregate = AggregateRef::from_id::<Organization>(organization_id);
        let mut changes: Vec<_> = self
            .relationship_store
            .read_subjects_by_aggregate(
                uow,
                &aggregate,
                &OrganizationOwnerRelation::REF.into(),
                None,
            )
            .await?
            .into_iter()
            .map(|subject| {
                RelationshipChange::Delete(Relationship::new::<Organization>(
                    organization_id,
                    OrganizationOwnerRelation::REF,
                    subject,
                ))
            })
            .collect();

        changes.push(RelationshipChange::Upsert(
            Relationship::new::<Organization>(
                organization_id,
                OrganizationOwnerRelation::REF,
                Self::owner_subject(owner),
            ),
        ));

        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
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
        if !event.is_for_aggregate::<Organization>() {
            return Ok(());
        }

        let domain_event = event.try_into_domain_event::<Organization>()?;
        match domain_event.payload() {
            OrganizationEventPayload::Created { owner, .. } => {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(
                            Relationship::new::<Organization>(
                                domain_event.aggregate_id(),
                                OrganizationOwnerRelation::REF,
                                Self::owner_subject(*owner),
                            ),
                        )],
                    )
                    .await?;
            }
            OrganizationEventPayload::OwnershipTransferred { owner } => {
                self.replace_owner_relationships(uow, domain_event.aggregate_id(), *owner)
                    .await?;
            }
            _ => return Ok(()),
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
        Organization, OrganizationHandle, OrganizationName, OrganizationOwner,
        OrganizationProfile, User, UserId,
    };

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
        current_subjects: Arc<Mutex<Vec<RelationshipSubject>>>,
    }

    impl TestRelationshipStore {
        fn recorded_changes(&self) -> Vec<RelationshipChange> {
            self.changes.lock().expect("lock should succeed").clone()
        }

        fn with_current_subjects(subjects: Vec<RelationshipSubject>) -> Self {
            Self {
                changes: Arc::default(),
                current_subjects: Arc::new(Mutex::new(subjects)),
            }
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
            Ok(self
                .current_subjects
                .lock()
                .expect("lock should succeed")
                .clone())
        }
    }

    fn created_event_envelope(owner: OrganizationOwner) -> EventEnvelope {
        let mut organization = Organization::default();
        organization
            .create(
                owner,
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationProfile::new(
                    OrganizationName::try_from("Acme Labs").expect("name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("creation should succeed");

        let event = organization
            .uncommitted_events()
            .first()
            .expect("created event should exist")
            .clone();
        let message_id = MessageId::new();

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
                Principal::System,
            )
            .expect("request context should be valid"),
        }
    }

    fn removed_event_envelope() -> EventEnvelope {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationOwner::User(UserId::new()),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationProfile::new(
                    OrganizationName::try_from("Acme Labs").expect("name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("creation should succeed");
        organization.remove().expect("remove should succeed");

        let event = organization
            .uncommitted_events()
            .last()
            .expect("removed event should exist")
            .clone();
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
                message_id,
                Principal::System,
            )
            .expect("request context should be valid"),
        }
    }

    fn ownership_transferred_event_envelope(
        previous_owner: UserId,
        next_owner: UserId,
    ) -> EventEnvelope {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationOwner::User(previous_owner),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationProfile::new(
                    OrganizationName::try_from("Acme Labs").expect("name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("creation should succeed");
        organization
            .transfer_ownership(OrganizationOwner::User(next_owner))
            .expect("ownership transfer should succeed");

        let event = organization
            .uncommitted_events()
            .last()
            .expect("ownership transferred event should exist")
            .clone();
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
                message_id,
                Principal::System,
            )
            .expect("request context should be valid"),
        }
    }

    #[tokio::test]
    async fn project_created_event_upserts_owner_relationship() {
        let store = TestRelationshipStore::default();
        let projector = OrganizationOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let owner = OrganizationOwner::User(UserId::new());
        let event = created_event_envelope(owner);

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
            RelationRefOwned::from(OrganizationOwnerRelation::REF)
        );
        let OrganizationOwner::User(owner) = owner;
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(owner))
        );
    }

    #[tokio::test]
    async fn project_removed_event_is_a_no_op() {
        let store = TestRelationshipStore::default();
        let projector = OrganizationOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let event = removed_event_envelope();

        projector
            .project(&mut uow, &event)
            .await
            .expect("projection should succeed");

        assert!(store.recorded_changes().is_empty());
    }

    #[tokio::test]
    async fn project_ownership_transferred_event_replaces_owner_relationship() {
        let previous_owner = UserId::new();
        let next_owner = UserId::new();
        let store =
            TestRelationshipStore::with_current_subjects(vec![RelationshipSubject::aggregate::<
                User,
            >(previous_owner)]);
        let projector = OrganizationOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let event = ownership_transferred_event_envelope(previous_owner, next_owner);

        projector
            .project(&mut uow, &event)
            .await
            .expect("projection should succeed");

        let changes = store.recorded_changes();
        assert_eq!(changes.len(), 2);

        let RelationshipChange::Delete(relationship) = &changes[0] else {
            panic!("expected delete relationship");
        };
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(previous_owner))
        );

        let RelationshipChange::Upsert(relationship) = &changes[1] else {
            panic!("expected upsert relationship");
        };
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(next_owner))
        );
    }
}
