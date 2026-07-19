use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{PublicOrganizationListUpsert, PublicOrganizationListWriter};

use super::{PublicOrganizationListProjectorError, PublicOrganizationListProjectorSpec};

/// Projects organization events into public organization list read models.
pub struct PublicOrganizationListProjector<W>
where
    W: PublicOrganizationListWriter,
{
    writer: W,
}

impl<W> PublicOrganizationListProjector<W>
where
    W: PublicOrganizationListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for PublicOrganizationListProjector<W>
where
    W: PublicOrganizationListWriter,
{
    type Spec = PublicOrganizationListProjectorSpec;
    type Uow = W::Uow;
    type Error = PublicOrganizationListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);
        let domain_event = event.try_into_domain_event::<Organization>()?;
        let organization_id = domain_event.aggregate_id();

        match domain_event.payload() {
            OrganizationEventPayload::Created {
                handle,
                display_name,
                picture,
                ..
            } => {
                self.writer
                    .upsert_organization(
                        uow,
                        event_context,
                        PublicOrganizationListUpsert {
                            id: organization_id,
                            handle: handle.clone(),
                            display_name: display_name.clone(),
                            picture: picture.clone(),
                        },
                    )
                    .await?;
            }
            OrganizationEventPayload::HandleChanged { handle } => {
                self.writer
                    .update_handle(uow, event_context, organization_id, handle.clone())
                    .await?;
            }
            OrganizationEventPayload::DisplayNameChanged { display_name } => {
                self.writer
                    .update_display_name(uow, event_context, organization_id, display_name.clone())
                    .await?;
            }
            OrganizationEventPayload::PictureChanged { picture, .. } => {
                self.writer
                    .update_picture(uow, event_context, organization_id, picture.clone())
                    .await?;
            }
            OrganizationEventPayload::Removed => {
                self.writer
                    .delete_organization(uow, event_context, organization_id)
                    .await?;
            }
            OrganizationEventPayload::CreateRejected { .. }
            | OrganizationEventPayload::OwnershipTransferred { .. }
            | OrganizationEventPayload::OwnershipTransferRejected { .. }
            | OrganizationEventPayload::HandleChangeRejected { .. }
            | OrganizationEventPayload::DisplayNameChangeRejected { .. }
            | OrganizationEventPayload::DescriptionChanged { .. }
            | OrganizationEventPayload::DescriptionChangeRejected { .. }
            | OrganizationEventPayload::WebsiteUrlChanged { .. }
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

    use appletheia::application::authorization::AggregateRef;
    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia::application::projection::Projector;
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{
        Aggregate, AggregateId, AggregateVersion, EventId, EventOccurredAt, EventPayload,
    };
    use banking_iam_domain::{
        Organization, OrganizationDisplayName, OrganizationEventPayload, OrganizationHandle,
        OrganizationId, OrganizationPictureRef, User, UserId,
    };
    use banking_shared_kernel_application::read_model::ReadModelEventContext;
    use uuid::Uuid;

    use crate::read_model::{
        PublicOrganizationListUpsert, PublicOrganizationListWriter,
        PublicOrganizationListWriterError,
    };

    use super::PublicOrganizationListProjector;

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
    struct TestPublicOrganizationListWriter {
        deleted_organization_ids: Arc<Mutex<Vec<OrganizationId>>>,
    }

    impl PublicOrganizationListWriter for TestPublicOrganizationListWriter {
        type Uow = TestUow;

        async fn upsert_organization(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _upsert: PublicOrganizationListUpsert,
        ) -> Result<(), PublicOrganizationListWriterError> {
            Ok(())
        }

        async fn update_handle(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _id: OrganizationId,
            _handle: OrganizationHandle,
        ) -> Result<(), PublicOrganizationListWriterError> {
            Ok(())
        }

        async fn update_display_name(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _id: OrganizationId,
            _display_name: OrganizationDisplayName,
        ) -> Result<(), PublicOrganizationListWriterError> {
            Ok(())
        }

        async fn update_picture(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _id: OrganizationId,
            _picture: Option<OrganizationPictureRef>,
        ) -> Result<(), PublicOrganizationListWriterError> {
            Ok(())
        }

        async fn delete_organization(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            id: OrganizationId,
        ) -> Result<(), PublicOrganizationListWriterError> {
            self.deleted_organization_ids.lock().expect("lock").push(id);
            Ok(())
        }
    }

    fn request_context(correlation_id: CorrelationId) -> RequestContext {
        let subject = AggregateRef::from_id::<User>(UserId::new());

        RequestContext::new(
            correlation_id,
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn removed_event_envelope(organization_id: OrganizationId) -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let payload = OrganizationEventPayload::Removed;

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Organization::TYPE),
            aggregate_id: AggregateIdValue::from(organization_id.value()),
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(
                payload.into_json_value().expect("payload should serialize"),
            )
            .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(MessageId::new()),
            context: request_context(correlation_id),
        }
    }

    #[tokio::test]
    async fn removed_event_physically_deletes_the_list_item() {
        let writer = TestPublicOrganizationListWriter::default();
        let deleted_organization_ids = Arc::clone(&writer.deleted_organization_ids);
        let projector = PublicOrganizationListProjector::new(writer);
        let organization_id = OrganizationId::new();

        projector
            .project(&mut TestUow, &removed_event_envelope(organization_id))
            .await
            .expect("removed event should be projected");

        assert_eq!(
            *deleted_organization_ids.lock().expect("lock"),
            vec![organization_id]
        );
    }
}
