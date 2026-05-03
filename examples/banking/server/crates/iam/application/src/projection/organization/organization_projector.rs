use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload};

use super::{OrganizationProjectorError, OrganizationProjectorSpec};
use crate::projection::{OrganizationProjectionStore, OrganizationProjectionUpsert};

/// Projects organization events into normalized organization projections.
pub struct OrganizationProjector<VS>
where
    VS: OrganizationProjectionStore,
{
    projection_store: VS,
}

impl<VS> OrganizationProjector<VS>
where
    VS: OrganizationProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for OrganizationProjector<VS>
where
    VS: OrganizationProjectionStore,
{
    type Spec = OrganizationProjectorSpec;
    type Uow = VS::Uow;
    type Error = OrganizationProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<Organization>()?;
        let organization_id = domain_event.aggregate_id();

        match domain_event.payload() {
            OrganizationEventPayload::Created {
                owner,
                handle,
                display_name,
                description,
                website_url,
                picture,
                ..
            } => {
                self.projection_store
                    .upsert(
                        uow,
                        OrganizationProjectionUpsert {
                            id: organization_id,
                            owner: *owner,
                            handle: handle.clone(),
                            display_name: display_name.clone(),
                            description: description.clone(),
                            website_url: website_url.clone(),
                            picture: picture.clone(),
                        },
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::OwnershipTransferred { owner } => {
                self.projection_store
                    .update_owner(
                        uow,
                        organization_id,
                        *owner,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::HandleChanged { handle } => {
                self.projection_store
                    .update_handle(
                        uow,
                        organization_id,
                        handle.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::DisplayNameChanged { display_name } => {
                self.projection_store
                    .update_display_name(
                        uow,
                        organization_id,
                        display_name.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::DescriptionChanged { description } => {
                self.projection_store
                    .update_description(
                        uow,
                        organization_id,
                        description.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::WebsiteUrlChanged { website_url } => {
                self.projection_store
                    .update_website_url(
                        uow,
                        organization_id,
                        website_url.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::PictureChanged { picture, .. } => {
                self.projection_store
                    .update_picture(
                        uow,
                        organization_id,
                        picture.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::Removed => {
                self.projection_store
                    .delete(
                        uow,
                        organization_id,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationEventPayload::OwnershipTransferRejected { .. }
            | OrganizationEventPayload::HandleChangeRejected { .. }
            | OrganizationEventPayload::DisplayNameChangeRejected { .. }
            | OrganizationEventPayload::DescriptionChangeRejected { .. }
            | OrganizationEventPayload::WebsiteUrlChangeRejected { .. }
            | OrganizationEventPayload::PictureChangeRejected { .. }
            | OrganizationEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventNameOwned, EventSequence, SerializedEventPayload,
    };
    use appletheia::application::projection::Projector;
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateId, Event, EventOccurredAt, EventPayload};
    use banking_iam_domain::{
        Organization, OrganizationDescription, OrganizationDisplayName, OrganizationEventPayload,
        OrganizationHandle, OrganizationId, OrganizationOwner, OrganizationPictureRef,
        OrganizationWebsiteUrl, UserId,
    };

    use super::OrganizationProjector;
    use crate::projection::{
        OrganizationProjectionStore, OrganizationProjectionStoreError, OrganizationProjectionUpsert,
    };

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

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum RecordedChange {
        Upsert(Box<OrganizationProjectionUpsert>, EventSequence),
        Owner(OrganizationId, OrganizationOwner, EventSequence),
        Delete(OrganizationId, EventSequence),
    }

    #[derive(Clone, Default)]
    struct TestOrganizationProjectionStore {
        changes: Arc<Mutex<Vec<RecordedChange>>>,
    }

    impl TestOrganizationProjectionStore {
        fn recorded_changes(&self) -> Vec<RecordedChange> {
            self.changes.lock().expect("lock should succeed").clone()
        }
    }

    impl OrganizationProjectionStore for TestOrganizationProjectionStore {
        type Uow = TestUow;

        async fn upsert(
            &self,
            _uow: &mut Self::Uow,
            input: OrganizationProjectionUpsert,
            event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            self.changes
                .lock()
                .expect("lock should succeed")
                .push(RecordedChange::Upsert(Box::new(input), event_sequence));
            Ok(())
        }

        async fn update_handle(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _handle: OrganizationHandle,
            _event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            Ok(())
        }

        async fn update_owner(
            &self,
            _uow: &mut Self::Uow,
            id: OrganizationId,
            owner: OrganizationOwner,
            event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            self.changes
                .lock()
                .expect("lock should succeed")
                .push(RecordedChange::Owner(id, owner, event_sequence));
            Ok(())
        }

        async fn update_display_name(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _display_name: OrganizationDisplayName,
            _event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            Ok(())
        }

        async fn update_description(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _description: Option<OrganizationDescription>,
            _event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            Ok(())
        }

        async fn update_website_url(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _website_url: Option<OrganizationWebsiteUrl>,
            _event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            Ok(())
        }

        async fn update_picture(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _picture: Option<OrganizationPictureRef>,
            _event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            Ok(())
        }

        async fn delete(
            &self,
            _uow: &mut Self::Uow,
            id: OrganizationId,
            event_sequence: EventSequence,
            _occurred_at: EventOccurredAt,
        ) -> Result<(), OrganizationProjectionStoreError> {
            self.changes
                .lock()
                .expect("lock should succeed")
                .push(RecordedChange::Delete(id, event_sequence));
            Ok(())
        }
    }

    fn event_envelope(
        event: &Event<OrganizationId, OrganizationEventPayload>,
        event_sequence: i64,
    ) -> appletheia::application::event::EventEnvelope {
        let message_id = MessageId::new();

        appletheia::application::event::EventEnvelope {
            event_sequence: EventSequence::try_from(event_sequence)
                .expect("sequence should be valid"),
            event_id: event.id(),
            aggregate_type: AggregateTypeOwned::from(Organization::TYPE),
            aggregate_id: AggregateIdValue::from(event.aggregate_id().value()),
            aggregate_version: event.aggregate_version(),
            event_name: EventNameOwned::from(event.payload().name()),
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

    fn organization() -> Organization {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationOwner::User(UserId::new()),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationDisplayName::try_from("Acme Labs")
                    .expect("display name should be valid"),
                None,
                None,
                None,
            )
            .expect("organization creation should succeed");
        organization
    }

    #[tokio::test]
    async fn created_event_upserts_organization_projection() {
        let store = TestOrganizationProjectionStore::default();
        let projector = OrganizationProjector::new(store.clone());
        let organization = organization();
        let event = organization
            .uncommitted_events()
            .first()
            .expect("created event should exist");
        let envelope = event_envelope(event, 1);

        projector
            .project(&mut TestUow, &envelope)
            .await
            .expect("projection should succeed");

        let changes = store.recorded_changes();
        assert_eq!(changes.len(), 1);

        let RecordedChange::Upsert(projection, sequence) = &changes[0] else {
            panic!("change should be upsert");
        };
        assert_eq!(projection.id, event.aggregate_id());
        assert!(matches!(projection.owner, OrganizationOwner::User(_)));
        assert_eq!(projection.handle.value(), "acme-labs");
        assert_eq!(projection.display_name.value(), "Acme Labs");
        assert_eq!(
            *sequence,
            EventSequence::try_from(1).expect("sequence should be valid")
        );
    }

    #[tokio::test]
    async fn ownership_transferred_event_updates_owner() {
        let store = TestOrganizationProjectionStore::default();
        let projector = OrganizationProjector::new(store.clone());
        let mut organization = organization();
        let new_owner = OrganizationOwner::User(UserId::new());
        organization
            .transfer_ownership(new_owner)
            .expect("ownership transfer should succeed");
        let event = organization
            .uncommitted_events()
            .last()
            .expect("ownership transferred event should exist");
        let envelope = event_envelope(event, 2);

        projector
            .project(&mut TestUow, &envelope)
            .await
            .expect("projection should succeed");

        assert_eq!(
            store.recorded_changes(),
            vec![RecordedChange::Owner(
                event.aggregate_id(),
                new_owner,
                EventSequence::try_from(2).expect("sequence should be valid"),
            )]
        );
    }

    #[tokio::test]
    async fn removed_event_deletes_organization_projection() {
        let store = TestOrganizationProjectionStore::default();
        let projector = OrganizationProjector::new(store.clone());
        let mut organization = organization();
        organization
            .remove()
            .expect("organization removal should succeed");
        let event = organization
            .uncommitted_events()
            .last()
            .expect("removed event should exist");
        let envelope = event_envelope(event, 2);

        projector
            .project(&mut TestUow, &envelope)
            .await
            .expect("projection should succeed");

        assert_eq!(
            store.recorded_changes(),
            vec![RecordedChange::Delete(
                event.aggregate_id(),
                EventSequence::try_from(2).expect("sequence should be valid"),
            )]
        );
    }
}
