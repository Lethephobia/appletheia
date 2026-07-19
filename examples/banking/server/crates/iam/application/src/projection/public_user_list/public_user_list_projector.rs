use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{PublicUserListItemStatus, PublicUserListUpsert, PublicUserListWriter};

use super::{PublicUserListProjectorError, PublicUserListProjectorSpec};

/// Projects user events into public user list read models.
pub struct PublicUserListProjector<W>
where
    W: PublicUserListWriter,
{
    writer: W,
}

impl<W> PublicUserListProjector<W>
where
    W: PublicUserListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for PublicUserListProjector<W>
where
    W: PublicUserListWriter,
{
    type Spec = PublicUserListProjectorSpec;
    type Uow = W::Uow;
    type Error = PublicUserListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);
        let domain_event = event.try_into_domain_event::<User>()?;
        let user_id = domain_event.aggregate_id();

        match domain_event.payload() {
            UserEventPayload::Registered { .. } => {
                self.writer
                    .upsert_user(
                        uow,
                        event_context,
                        PublicUserListUpsert {
                            id: user_id,
                            username: None,
                            display_name: None,
                            picture: None,
                            status: PublicUserListItemStatus::Active,
                        },
                    )
                    .await?;
            }
            UserEventPayload::UsernameChanged { username } => {
                self.writer
                    .update_username(uow, event_context, user_id, username.clone())
                    .await?;
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                self.writer
                    .update_display_name(uow, event_context, user_id, display_name.clone())
                    .await?;
            }
            UserEventPayload::PictureChanged { picture, .. } => {
                self.writer
                    .update_picture(uow, event_context, user_id, picture.clone())
                    .await?;
            }
            UserEventPayload::Activated => {
                self.writer
                    .update_status(
                        uow,
                        event_context,
                        user_id,
                        PublicUserListItemStatus::Active,
                    )
                    .await?;
            }
            UserEventPayload::Deactivated => {
                self.writer
                    .update_status(
                        uow,
                        event_context,
                        user_id,
                        PublicUserListItemStatus::Inactive,
                    )
                    .await?;
            }
            UserEventPayload::Removed => {
                self.writer.delete_user(uow, event_context, user_id).await?;
            }
            UserEventPayload::IdentityLinked { .. }
            | UserEventPayload::IdentityLinkRejected { .. }
            | UserEventPayload::IdentityEmailChanged { .. }
            | UserEventPayload::IdentityEmailChangeRejected { .. }
            | UserEventPayload::UsernameChangeRejected { .. }
            | UserEventPayload::DisplayNameChangeRejected { .. }
            | UserEventPayload::BioChanged { .. }
            | UserEventPayload::BioChangeRejected { .. }
            | UserEventPayload::PictureChangeRejected { .. }
            | UserEventPayload::ActivateRejected { .. }
            | UserEventPayload::DeactivateRejected { .. }
            | UserEventPayload::RemoveRejected { .. }
            | UserEventPayload::OrganizationMembershipGranted { .. }
            | UserEventPayload::OrganizationMembershipGrantRejected { .. }
            | UserEventPayload::OrganizationMembershipRolesChanged { .. }
            | UserEventPayload::OrganizationMembershipRolesChangeRejected { .. }
            | UserEventPayload::OrganizationMembershipRemoved { .. }
            | UserEventPayload::OrganizationMembershipRemoveRejected { .. } => {}
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
        User, UserDisplayName, UserEventPayload, UserId, UserPictureRef, Username,
    };
    use banking_shared_kernel_application::read_model::ReadModelEventContext;
    use uuid::Uuid;

    use crate::read_model::{
        PublicUserListItemStatus, PublicUserListUpsert, PublicUserListWriter,
        PublicUserListWriterError,
    };

    use super::PublicUserListProjector;

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
    struct TestPublicUserListWriter {
        deleted_user_ids: Arc<Mutex<Vec<UserId>>>,
    }

    impl PublicUserListWriter for TestPublicUserListWriter {
        type Uow = TestUow;

        async fn upsert_user(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _upsert: PublicUserListUpsert,
        ) -> Result<(), PublicUserListWriterError> {
            Ok(())
        }

        async fn update_username(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _id: UserId,
            _username: Username,
        ) -> Result<(), PublicUserListWriterError> {
            Ok(())
        }

        async fn update_display_name(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _id: UserId,
            _display_name: UserDisplayName,
        ) -> Result<(), PublicUserListWriterError> {
            Ok(())
        }

        async fn update_picture(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _id: UserId,
            _picture: Option<UserPictureRef>,
        ) -> Result<(), PublicUserListWriterError> {
            Ok(())
        }

        async fn update_status(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            _id: UserId,
            _status: PublicUserListItemStatus,
        ) -> Result<(), PublicUserListWriterError> {
            Ok(())
        }

        async fn delete_user(
            &self,
            _uow: &mut Self::Uow,
            _event_context: ReadModelEventContext,
            id: UserId,
        ) -> Result<(), PublicUserListWriterError> {
            self.deleted_user_ids.lock().expect("lock").push(id);
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

    fn removed_event_envelope(user_id: UserId) -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let payload = UserEventPayload::Removed;

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(User::TYPE),
            aggregate_id: AggregateIdValue::from(user_id.value()),
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
        let writer = TestPublicUserListWriter::default();
        let deleted_user_ids = Arc::clone(&writer.deleted_user_ids);
        let projector = PublicUserListProjector::new(writer);
        let user_id = UserId::new();

        projector
            .project(&mut TestUow, &removed_event_envelope(user_id))
            .await
            .expect("removed event should be projected");

        assert_eq!(*deleted_user_ids.lock().expect("lock"), vec![user_id]);
    }
}
