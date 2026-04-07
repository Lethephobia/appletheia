use appletheia::application::authorization::{
    AggregateRef, Relation, RelationRefOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, User};
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
                let subject = match owner {
                    banking_ledger_domain::account::AccountOwner::User(user_id) => {
                        RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(*user_id))
                    }
                    banking_ledger_domain::account::AccountOwner::Organization(organization_id) => {
                        RelationshipSubject::Aggregate(AggregateRef::from_id::<Organization>(
                            *organization_id,
                        ))
                    }
                };

                self.relationship_store
                    .apply_changes(
                        uow,
                        &[RelationshipChange::Upsert(Relationship {
                            aggregate: AggregateRef::from_id::<Account>(event.aggregate_id()),
                            relation: RelationRefOwned::from(AccountOwnerRelation::REF),
                            subject,
                        })],
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
    use banking_iam_domain::{Organization, OrganizationId, User, UserId};
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

    fn opened_event_envelope_for_organization_owner() -> (EventEnvelope, OrganizationId) {
        let mut account = Account::default();
        let organization_id = OrganizationId::new();
        account
            .open(
                AccountOwner::from(organization_id),
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
            organization_id,
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
            RelationRefOwned::from(AccountOwnerRelation::REF)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(user_id))
        );
    }

    #[tokio::test]
    async fn project_opened_event_upserts_owner_relationship_for_organization_owner() {
        let (event, organization_id) = opened_event_envelope_for_organization_owner();
        let store = TestRelationshipStore::default();
        let projector = AccountOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;

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
            RelationRefOwned::from(AccountOwnerRelation::REF)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<Organization>(organization_id))
        );
    }
}
