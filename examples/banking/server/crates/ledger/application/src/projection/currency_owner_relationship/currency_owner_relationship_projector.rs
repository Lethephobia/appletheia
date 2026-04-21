use appletheia::application::authorization::{
    AggregateRef, Relation, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload, CurrencyOwner};

use super::{CurrencyOwnerRelationshipProjectorError, CurrencyOwnerRelationshipProjectorSpec};
use crate::authorization::CurrencyOwnerRelation;

/// Projects the owner relationship for currencies.
pub struct CurrencyOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> CurrencyOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn owner_subject(owner: CurrencyOwner) -> RelationshipSubject {
        match owner {
            CurrencyOwner::User(user_id) => RelationshipSubject::aggregate::<User>(user_id),
            CurrencyOwner::Organization(organization_id) => {
                RelationshipSubject::aggregate::<Organization>(organization_id)
            }
        }
    }

    async fn replace_owner_relationships(
        &self,
        uow: &mut RS::Uow,
        currency_id: banking_ledger_domain::currency::CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<(), CurrencyOwnerRelationshipProjectorError> {
        let aggregate = AggregateRef::from_id::<Currency>(currency_id);
        let mut changes: Vec<_> = self
            .relationship_store
            .read_subjects_by_aggregate(uow, &aggregate, &CurrencyOwnerRelation::REF.into(), None)
            .await?
            .into_iter()
            .map(|subject| {
                RelationshipChange::Delete(Relationship::new::<Currency>(
                    currency_id,
                    CurrencyOwnerRelation::REF,
                    subject,
                ))
            })
            .collect();

        changes.push(RelationshipChange::Upsert(Relationship::new::<Currency>(
            currency_id,
            CurrencyOwnerRelation::REF,
            Self::owner_subject(owner),
        )));

        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }
}

impl<RS> Projector for CurrencyOwnerRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = CurrencyOwnerRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = CurrencyOwnerRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Currency>() {
            let domain_event = event.try_into_domain_event::<Currency>()?;
            match domain_event.payload() {
                CurrencyEventPayload::Defined { owner, .. } => {
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Upsert(Relationship::new::<Currency>(
                                domain_event.aggregate_id(),
                                CurrencyOwnerRelation::REF,
                                Self::owner_subject(*owner),
                            ))],
                        )
                        .await?;
                }
                CurrencyEventPayload::OwnershipTransferred { owner } => {
                    self.replace_owner_relationships(uow, domain_event.aggregate_id(), *owner)
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
    use banking_iam_domain::{Organization, User};
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyName, CurrencyOwner, CurrencySymbol,
    };

    use super::CurrencyOwnerRelationshipProjector;
    use crate::authorization::CurrencyOwnerRelation;

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

    fn defined_event_envelope(owner: CurrencyOwner) -> EventEnvelope {
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        let event = currency
            .uncommitted_events()
            .first()
            .expect("defined event should exist")
            .clone();
        let message_id = MessageId::new();

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: event.id(),
            aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                Currency::TYPE,
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

    fn ownership_transferred_event_envelope() -> (
        EventEnvelope,
        banking_iam_domain::OrganizationId,
        banking_iam_domain::UserId,
    ) {
        let mut currency = Currency::default();
        let previous_owner = banking_iam_domain::OrganizationId::new();
        let next_owner = banking_iam_domain::UserId::new();
        currency
            .define(
                CurrencyOwner::Organization(previous_owner),
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");
        currency
            .transfer_ownership(CurrencyOwner::User(next_owner))
            .expect("ownership transfer should succeed");

        let event = currency
            .uncommitted_events()
            .last()
            .expect("ownership transferred event should exist")
            .clone();
        let message_id = MessageId::new();

        (
            EventEnvelope {
                event_sequence: EventSequence::try_from(2).expect("sequence should be valid"),
                event_id: event.id(),
                aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                    Currency::TYPE,
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
            previous_owner,
            next_owner,
        )
    }

    #[tokio::test]
    async fn project_defined_event_upserts_owner_relationship_for_user_owner() {
        let store = TestRelationshipStore::default();
        let projector = CurrencyOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let user_id = banking_iam_domain::UserId::new();
        let event = defined_event_envelope(CurrencyOwner::User(user_id));

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
            RelationRefOwned::from(CurrencyOwnerRelation::REF)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(user_id))
        );
    }

    #[tokio::test]
    async fn project_defined_event_upserts_owner_relationship_for_organization_owner() {
        let store = TestRelationshipStore::default();
        let projector = CurrencyOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let organization_id = banking_iam_domain::OrganizationId::new();
        let event = defined_event_envelope(CurrencyOwner::Organization(organization_id));

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
            RelationRefOwned::from(CurrencyOwnerRelation::REF)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<Organization>(organization_id))
        );
    }

    #[tokio::test]
    async fn project_ownership_transferred_event_replaces_owner_relationship() {
        let (event, previous_owner, next_owner) = ownership_transferred_event_envelope();
        let store =
            TestRelationshipStore::with_current_subjects(vec![RelationshipSubject::aggregate::<
                Organization,
            >(previous_owner)]);
        let projector = CurrencyOwnerRelationshipProjector::new(store.clone());
        let mut uow = TestUow;

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
            RelationshipSubject::Aggregate(AggregateRef::from_id::<Organization>(previous_owner))
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
