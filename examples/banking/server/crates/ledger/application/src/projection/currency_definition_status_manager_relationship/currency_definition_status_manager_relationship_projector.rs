use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::currency_definition::{
    CurrencyDefinition, CurrencyDefinitionEventPayload,
};

use super::{
    CurrencyDefinitionStatusManagerRelationshipProjectorError,
    CurrencyDefinitionStatusManagerRelationshipProjectorSpec,
};
use crate::authorization::CurrencyDefinitionStatusManagerRelation;

/// Projects the initial status manager relationship for new currency definitions.
pub struct CurrencyDefinitionStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> CurrencyDefinitionStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for CurrencyDefinitionStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = CurrencyDefinitionStatusManagerRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = CurrencyDefinitionStatusManagerRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<CurrencyDefinition>()?;
        let CurrencyDefinitionEventPayload::Defined { .. } = domain_event.payload() else {
            return Ok(());
        };

        let appletheia::application::request_context::ActorRef::Subject { subject } =
            &event.context.actor
        else {
            return Ok(());
        };

        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship {
                    aggregate: AggregateRef::from_id::<CurrencyDefinition>(
                        domain_event.aggregate_id(),
                    ),
                    relation: RelationNameOwned::from(
                        CurrencyDefinitionStatusManagerRelation::NAME,
                    ),
                    subject: RelationshipSubject::Aggregate(subject.clone()),
                })],
            )
            .await?;

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
    use banking_ledger_domain::core::{CurrencyDecimals, CurrencySymbol};
    use banking_ledger_domain::currency_definition::{CurrencyDefinition, CurrencyName};
    use uuid::Uuid;

    use super::CurrencyDefinitionStatusManagerRelationshipProjector;
    use crate::authorization::CurrencyDefinitionStatusManagerRelation;

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
            Ok(Vec::new())
        }
    }

    fn defined_event_envelope() -> EventEnvelope {
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        let event = currency_definition
            .uncommitted_events()
            .first()
            .expect("defined event should exist")
            .clone();
        let message_id = MessageId::new();
        let actor_subject = AggregateRef::new(
            appletheia::application::event::AggregateTypeOwned::try_from("user")
                .expect("aggregate type should be valid"),
            appletheia::application::event::AggregateIdValue::from(Uuid::now_v7()),
        );

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: event.id(),
            aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                CurrencyDefinition::TYPE,
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
        }
    }

    #[tokio::test]
    async fn project_defined_event_upserts_status_manager_relationship() {
        let store = TestRelationshipStore::default();
        let projector = CurrencyDefinitionStatusManagerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let event = defined_event_envelope();

        projector
            .project(&mut uow, &event)
            .await
            .expect("projection should succeed");

        let changes = store.recorded_changes();
        assert_eq!(changes.len(), 1);

        let RelationshipChange::Upsert(relationship) = &changes[0] else {
            panic!("expected upsert relationship");
        };

        let expected_subject = match &event.context.actor {
            ActorRef::Subject { subject } => RelationshipSubject::Aggregate(subject.clone()),
            _ => panic!("expected subject actor"),
        };

        assert_eq!(
            relationship.relation,
            RelationNameOwned::from(CurrencyDefinitionStatusManagerRelation::NAME)
        );
        assert_eq!(relationship.subject, expected_subject);
    }
}
