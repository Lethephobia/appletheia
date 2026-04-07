use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::User;
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::{AccountOwnerRelationshipProjectorError, AccountOwnerRelationshipProjectorSpec};
use crate::authorization::AccountOwnerRelation;

/// Projects the owner relationship for accounts.
pub struct AccountOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> AccountOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    async fn delete_owner_relationships(
        &self,
        uow: &mut RS::Uow,
        account: AggregateRef,
    ) -> Result<(), AccountOwnerRelationshipProjectorError> {
        let relation = RelationNameOwned::from(AccountOwnerRelation::NAME);
        let subjects = self
            .relationship_store
            .read_subjects_by_aggregate(uow, &account, &relation)
            .await?;

        let changes = subjects
            .into_iter()
            .map(|subject| {
                RelationshipChange::Delete(Relationship {
                    aggregate: account.clone(),
                    relation: relation.clone(),
                    subject,
                })
            })
            .collect::<Vec<_>>();

        if !changes.is_empty() {
            self.relationship_store.apply_changes(uow, &changes).await?;
        }

        Ok(())
    }
}

impl<RS> Projector for AccountOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = AccountOwnerRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = AccountOwnerRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event = event.try_into_domain_event::<Account>()?;
        match event.payload() {
            AccountEventPayload::Opened { owner, .. } => {
                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship {
                            aggregate: AggregateRef::from_id::<Account>(event.aggregate_id()),
                            relation: RelationNameOwned::from(AccountOwnerRelation::NAME),
                            subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(
                                *owner.user_id(),
                            )),
                        })],
                    )
                    .await?;
            }
            AccountEventPayload::Closed => {
                self.delete_owner_relationships(
                    uow,
                    AggregateRef::from_id::<Account>(event.aggregate_id()),
                )
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
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{Account, AccountName, AccountOwner};
    use banking_ledger_domain::currency_definition::CurrencyDefinitionId;

    use super::AccountOwnerRelationshipProjector;
    use crate::authorization::AccountOwnerRelation;

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
        owner_subjects_by_aggregate: Arc<Mutex<Vec<RelationshipSubject>>>,
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
            relation: &RelationNameOwned,
        ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
            if relation == &RelationNameOwned::from(AccountOwnerRelation::NAME) {
                Ok(self
                    .owner_subjects_by_aggregate
                    .lock()
                    .expect("lock should succeed")
                    .clone())
            } else {
                Ok(Vec::new())
            }
        }
    }

    fn opened_event_envelope() -> (EventEnvelope, UserId) {
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
            .first()
            .expect("opened event should exist")
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
                    ActorRef::System,
                    Principal::Authenticated { subject },
                ),
            },
            user_id,
        )
    }

    fn closed_event_envelope() -> (EventEnvelope, UserId) {
        let mut account = Account::default();
        let user_id = UserId::new();
        account
            .open(
                AccountOwner::from(user_id),
                account_name(),
                CurrencyDefinitionId::new(),
            )
            .expect("open should succeed");
        account.close().expect("close should succeed");

        let event = account
            .uncommitted_events()
            .get(1)
            .expect("closed event should exist")
            .clone();
        let message_id = MessageId::new();

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
                    ActorRef::System,
                    Principal::System,
                ),
            },
            user_id,
        )
    }

    #[tokio::test]
    async fn project_opened_event_upserts_owner_relationship() {
        let store = TestRelationshipStore::default();
        let projector = AccountOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let (event, user_id) = opened_event_envelope();

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
            RelationNameOwned::from(AccountOwnerRelation::NAME)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(user_id))
        );
    }

    #[tokio::test]
    async fn project_closed_event_deletes_owner_relationship() {
        let (event, user_id) = closed_event_envelope();
        let store = TestRelationshipStore {
            owner_subjects_by_aggregate: Arc::new(Mutex::new(vec![
                RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(user_id)),
            ])),
            ..Default::default()
        };
        let projector = AccountOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;

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
            RelationNameOwned::from(AccountOwnerRelation::NAME)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(user_id))
        );
    }
}
