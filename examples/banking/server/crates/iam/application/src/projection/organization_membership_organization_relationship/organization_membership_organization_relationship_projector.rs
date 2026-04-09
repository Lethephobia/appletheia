use appletheia::application::authorization::{
    AggregateRef, Relation, RelationRefOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationMembership, OrganizationMembershipEventPayload,
};

use super::{
    OrganizationMembershipOrganizationRelationshipProjectorError,
    OrganizationMembershipOrganizationRelationshipProjectorSpec,
};
use crate::authorization::OrganizationMembershipOrganizationRelation;

/// Projects the organization relation for organization memberships.
pub struct OrganizationMembershipOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationMembershipOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> Projector for OrganizationMembershipOrganizationRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationMembershipOrganizationRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationMembershipOrganizationRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<OrganizationMembership>() {
            let domain_event = event.try_into_domain_event::<OrganizationMembership>()?;

            match domain_event.payload() {
                OrganizationMembershipEventPayload::Created {
                    organization_id, ..
                } => {
                    self.relationship_store
                        .apply_changes(
                            uow,
                            &[RelationshipChange::Upsert(Relationship::new::<
                                OrganizationMembership,
                            >(
                                domain_event.aggregate_id(),
                                OrganizationMembershipOrganizationRelation::REF,
                                RelationshipSubject::aggregate::<Organization>(*organization_id),
                            ))],
                        )
                        .await?;
                }
                OrganizationMembershipEventPayload::Removed { .. } => {
                    let aggregate = AggregateRef::from_id::<OrganizationMembership>(
                        domain_event.aggregate_id(),
                    );
                    let relation =
                        RelationRefOwned::from(OrganizationMembershipOrganizationRelation::REF);
                    let subjects = self
                        .relationship_store
                        .read_subjects_by_aggregate(uow, &aggregate, &relation, None)
                        .await?;

                    if subjects.is_empty() {
                        return Ok(());
                    }

                    let changes = subjects
                        .into_iter()
                        .map(|subject| {
                            RelationshipChange::Delete(Relationship::new::<OrganizationMembership>(
                                domain_event.aggregate_id(),
                                OrganizationMembershipOrganizationRelation::REF,
                                subject,
                            ))
                        })
                        .collect::<Vec<_>>();

                    self.relationship_store.apply_changes(uow, &changes).await?;
                }
                OrganizationMembershipEventPayload::Activated { .. } => return Ok(()),
                OrganizationMembershipEventPayload::Inactivated { .. } => return Ok(()),
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
    use banking_iam_domain::{Organization, OrganizationId, OrganizationMembership, UserId};

    use super::OrganizationMembershipOrganizationRelationshipProjector;
    use crate::authorization::OrganizationMembershipOrganizationRelation;

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
                .subjects_by_aggregate
                .lock()
                .expect("lock should succeed")
                .clone())
        }
    }

    fn created_event_envelope() -> (EventEnvelope, OrganizationId) {
        let mut membership = OrganizationMembership::default();
        let organization_id = OrganizationId::new();
        let user_id = UserId::new();
        membership
            .create(organization_id, user_id)
            .expect("creation should succeed");

        let event = membership
            .uncommitted_events()
            .first()
            .expect("created event should exist")
            .clone();
        let message_id = MessageId::new();

        (
            EventEnvelope {
                event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
                event_id: event.id(),
                aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                    OrganizationMembership::TYPE,
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
                    Principal::System,
                )
                .expect("request context should be valid"),
            },
            organization_id,
        )
    }

    fn removed_event_envelope() -> EventEnvelope {
        let mut membership = OrganizationMembership::default();
        membership
            .create(OrganizationId::new(), UserId::new())
            .expect("creation should succeed");
        membership.remove().expect("remove should succeed");

        let event = membership
            .uncommitted_events()
            .last()
            .expect("removed event should exist")
            .clone();
        let message_id = MessageId::new();

        EventEnvelope {
            event_sequence: EventSequence::try_from(2).expect("sequence should be valid"),
            event_id: event.id(),
            aggregate_type: appletheia::application::event::AggregateTypeOwned::from(
                OrganizationMembership::TYPE,
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
                Principal::System,
            )
            .expect("request context should be valid"),
        }
    }

    #[tokio::test]
    async fn project_created_event_upserts_membership_organization_relationship() {
        let store = TestRelationshipStore::default();
        let projector = OrganizationMembershipOrganizationRelationshipProjector::new(store.clone());
        let mut uow = TestUow;
        let (event, organization_id) = created_event_envelope();

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
            RelationRefOwned::from(OrganizationMembershipOrganizationRelation::REF)
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::Aggregate(AggregateRef::from_id::<Organization>(organization_id))
        );
    }

    #[tokio::test]
    async fn project_removed_event_deletes_membership_organization_relationship() {
        let store = TestRelationshipStore {
            subjects_by_aggregate: Arc::new(Mutex::new(vec![RelationshipSubject::Aggregate(
                AggregateRef::from_id::<Organization>(OrganizationId::new()),
            )])),
            ..Default::default()
        };
        let projector = OrganizationMembershipOrganizationRelationshipProjector::new(store.clone());
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
            RelationRefOwned::from(OrganizationMembershipOrganizationRelation::REF)
        );
    }
}
