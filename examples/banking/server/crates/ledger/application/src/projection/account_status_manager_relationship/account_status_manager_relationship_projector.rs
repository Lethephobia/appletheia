use appletheia::application::authorization::{
    AggregateRef, Relation, RelationRefOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::User;
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::{
    AccountStatusManagerRelationshipProjectorError, AccountStatusManagerRelationshipProjectorSpec,
};
use crate::authorization::AccountStatusManagerRelation;

/// Projects the status-manager relationship for accounts.
pub struct AccountStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> AccountStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for AccountStatusManagerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = AccountStatusManagerRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = AccountStatusManagerRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event = event.try_into_domain_event::<Account>()?;
        let AccountEventPayload::StatusManagerAssigned { status_manager } = event.payload() else {
            return Ok(());
        };

        let account = AggregateRef::from_id::<Account>(event.aggregate_id());
        let status_manager_subject = RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(
            *status_manager.user_id(),
        ));

        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship {
                    aggregate: account,
                    relation: RelationRefOwned::from(AccountStatusManagerRelation::REF),
                    subject: status_manager_subject,
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
        AggregateRef, Relation, RelationRefOwned, RelationshipChange, RelationshipStore,
        RelationshipStoreError, RelationshipSubject,
    };
    use appletheia::application::event::{EventEnvelope, EventSequence, SerializedEventPayload};
    use appletheia::application::projection::Projector;
    use appletheia::application::request_context::{
        ActorRef, CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{Account, AccountName, AccountOwner};
    use banking_ledger_domain::currency_definition::CurrencyDefinitionId;

    use super::AccountStatusManagerRelationshipProjector;
    use crate::authorization::AccountStatusManagerRelation;

    fn account_name() -> AccountName {
        AccountName::try_from("main").expect("account name should be valid")
    }

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

    fn status_manager_assigned_event_envelope() -> (EventEnvelope, UserId) {
        let mut account = Account::default();
        let user_id = UserId::new();
        account
            .open(
                AccountOwner::from(user_id),
                account_name(),
                CurrencyDefinitionId::new(),
            )
            .expect("open should succeed");

        let event = account
            .uncommitted_events()
            .get(2)
            .expect("status-manager-assigned event should exist")
            .clone();
        let message_id = MessageId::new();
        let subject = AggregateRef::from_id::<User>(user_id);

        (
            EventEnvelope {
                event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
                event_id: event.id(),
                aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                    Account::TYPE,
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
                    Principal::Authenticated { subject },
                )
                .expect("request context should be valid"),
            },
            user_id,
        )
    }

    #[tokio::test]
    async fn project_status_manager_assigned_event_upserts_status_manager_relationship() {
        let store = TestRelationshipStore::default();
        let projector = AccountStatusManagerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let (event, user_id) = status_manager_assigned_event_envelope();

        projector
            .project(&mut uow, &event)
            .await
            .expect("projection should succeed");

        let changes = store.recorded_changes();
        assert_eq!(changes.len(), 1);

        let expected_subject =
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(user_id));

        let relationship = match &changes[0] {
            RelationshipChange::Upsert(relationship) => relationship,
            _ => panic!("expected upsert relationship"),
        };

        assert_eq!(
            relationship.relation,
            RelationRefOwned::from(AccountStatusManagerRelation::REF)
        );
        assert_eq!(relationship.subject, expected_subject);
    }
}
