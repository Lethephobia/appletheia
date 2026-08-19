use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{
    MaterializationEventContext, ReadModelFragmentPartition, ReadModelPartition,
};
use banking_iam_domain::{User, UserEventPayload};

use crate::projection::{
    MaterializedUserStatus, UserFragment, UserFragmentUpsert, UserFragmentWriter,
};

use super::{UserFragmentProjectorError, UserFragmentProjectorSpec};

/// Projects public user fragments and emits their read-model protocol mappings.
pub struct UserFragmentProjector<W>
where
    W: UserFragmentWriter,
{
    user_fragment_writer: W,
}

impl<W> UserFragmentProjector<W>
where
    W: UserFragmentWriter,
{
    pub fn new(user_fragment_writer: W) -> Self {
        Self {
            user_fragment_writer,
        }
    }
}

impl<W> Projector for UserFragmentProjector<W>
where
    W: UserFragmentWriter,
{
    type Spec = UserFragmentProjectorSpec;
    type Fragment = UserFragment;
    type Uow = W::Uow;
    type Error = UserFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<Vec<ReadModelFragmentPartition<Self::Fragment>>, Self::Error> {
        let mut invalidated_partitions = Vec::new();
        let domain_event = event.try_into_domain_event::<User>()?;
        let user_id = domain_event.aggregate_id();

        let payload = domain_event.payload();
        match payload {
            UserEventPayload::Registered { .. } => {
                if let Some(fragment) = self
                    .user_fragment_writer
                    .upsert(
                        uow,
                        event_context,
                        UserFragmentUpsert {
                            id: user_id,
                            username: None,
                            display_name: None,
                            bio: None,
                            picture: None,
                            status: MaterializedUserStatus::Active,
                        },
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            UserEventPayload::UsernameChanged { username } => {
                if let Some(fragment) = self
                    .user_fragment_writer
                    .update_username(uow, event_context, user_id, username.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                if let Some(fragment) = self
                    .user_fragment_writer
                    .update_display_name(uow, event_context, user_id, display_name.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            UserEventPayload::BioChanged { bio } => {
                if let Some(fragment) = self
                    .user_fragment_writer
                    .update_bio(uow, event_context, user_id, bio.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            UserEventPayload::PictureChanged { picture, .. } => {
                if let Some(fragment) = self
                    .user_fragment_writer
                    .update_picture(uow, event_context, user_id, picture.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            UserEventPayload::Activated => {
                if let Some(fragment) = self
                    .user_fragment_writer
                    .update_status(uow, event_context, user_id, MaterializedUserStatus::Active)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            UserEventPayload::Deactivated => {
                if let Some(fragment) = self
                    .user_fragment_writer
                    .update_status(
                        uow,
                        event_context,
                        user_id,
                        MaterializedUserStatus::Inactive,
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            UserEventPayload::Removed => {
                if self
                    .user_fragment_writer
                    .delete(uow, event_context, user_id)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::new(user_id));
                }
            }
            UserEventPayload::IdentityLinked { .. }
            | UserEventPayload::IdentityLinkRejected { .. }
            | UserEventPayload::IdentityEmailChanged { .. }
            | UserEventPayload::IdentityEmailChangeRejected { .. }
            | UserEventPayload::UsernameChangeRejected { .. }
            | UserEventPayload::DisplayNameChangeRejected { .. }
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

        Ok(invalidated_partitions)
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
    use appletheia::application::projection::{Projector, ProjectorName};
    use appletheia::application::read_model::{
        MaterializationEventContext, ReadModelDependency, ReadModelFragmentPartition,
        ReadModelInvalidationEnvelope, ReadModelObservation,
    };
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{
        Aggregate, AggregateId, AggregateVersion, EventId, EventOccurredAt, EventPayload,
    };
    use banking_iam_domain::{
        User, UserBio, UserDisplayName, UserEventPayload, UserId, UserPictureRef, Username,
    };
    use uuid::Uuid;

    use super::UserFragmentProjector;
    use crate::projection::{
        MaterializedUserStatus, UserFragment, UserFragmentUpsert, UserFragmentWriter,
        UserFragmentWriterError,
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

    #[derive(Clone, Default)]
    struct TestUserFragmentWriter {
        deleted_user_ids: Arc<Mutex<Vec<UserId>>>,
    }

    impl TestUserFragmentWriter {
        fn fragment(
            event_context: MaterializationEventContext,
            user_id: UserId,
            status: MaterializedUserStatus,
        ) -> UserFragment {
            UserFragment {
                id: user_id,
                username: None,
                display_name: None,
                bio: None,
                picture: None,
                status,
                created_at: event_context.occurred_at,
                observation: ReadModelObservation::new(
                    event_context.event_id,
                    event_context.event_id,
                ),
            }
        }
    }

    impl UserFragmentWriter for TestUserFragmentWriter {
        type Uow = TestUow;

        async fn upsert(
            &self,
            _uow: &mut Self::Uow,
            event_context: MaterializationEventContext,
            upsert: UserFragmentUpsert,
        ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
            Ok(Some(UserFragment {
                id: upsert.id,
                username: upsert.username,
                display_name: upsert.display_name,
                bio: upsert.bio,
                picture: upsert.picture,
                status: upsert.status,
                created_at: event_context.occurred_at,
                observation: ReadModelObservation::new(
                    event_context.event_id,
                    event_context.event_id,
                ),
            }))
        }

        async fn update_username(
            &self,
            _uow: &mut Self::Uow,
            event_context: MaterializationEventContext,
            user_id: UserId,
            username: Username,
        ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
            let mut fragment =
                Self::fragment(event_context, user_id, MaterializedUserStatus::Active);
            fragment.username = Some(username);

            Ok(Some(fragment))
        }

        async fn update_display_name(
            &self,
            _uow: &mut Self::Uow,
            event_context: MaterializationEventContext,
            user_id: UserId,
            display_name: UserDisplayName,
        ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
            let mut fragment =
                Self::fragment(event_context, user_id, MaterializedUserStatus::Active);
            fragment.display_name = Some(display_name);

            Ok(Some(fragment))
        }

        async fn update_bio(
            &self,
            _uow: &mut Self::Uow,
            event_context: MaterializationEventContext,
            user_id: UserId,
            bio: Option<UserBio>,
        ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
            let mut fragment =
                Self::fragment(event_context, user_id, MaterializedUserStatus::Active);
            fragment.bio = bio;

            Ok(Some(fragment))
        }

        async fn update_picture(
            &self,
            _uow: &mut Self::Uow,
            event_context: MaterializationEventContext,
            user_id: UserId,
            picture: Option<UserPictureRef>,
        ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
            let mut fragment =
                Self::fragment(event_context, user_id, MaterializedUserStatus::Active);
            fragment.picture = picture;

            Ok(Some(fragment))
        }

        async fn update_status(
            &self,
            _uow: &mut Self::Uow,
            event_context: MaterializationEventContext,
            user_id: UserId,
            status: MaterializedUserStatus,
        ) -> Result<Option<UserFragment>, UserFragmentWriterError> {
            Ok(Some(Self::fragment(event_context, user_id, status)))
        }

        async fn delete(
            &self,
            _uow: &mut Self::Uow,
            _event_context: MaterializationEventContext,
            user_id: UserId,
        ) -> Result<bool, UserFragmentWriterError> {
            self.deleted_user_ids.lock().expect("lock").push(user_id);
            Ok(true)
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

    fn invalidation_envelope(
        event: &EventEnvelope,
        partitions: Vec<ReadModelFragmentPartition<UserFragment>>,
    ) -> ReadModelInvalidationEnvelope {
        let dependencies = partitions
            .into_iter()
            .map(|partition| {
                partition
                    .try_into_serialized::<UserFragment>()
                    .map(ReadModelDependency::Partition)
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("partitions should serialize");
        ReadModelInvalidationEnvelope::try_new(
            event,
            ProjectorName::new("user_fragment"),
            dependencies,
        )
        .expect("read-model invalidation should finalize")
    }

    fn activated_event_envelope(user_id: UserId) -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let payload = UserEventPayload::Activated;

        EventEnvelope {
            event_sequence: EventSequence::try_from(2).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(User::TYPE),
            aggregate_id: AggregateIdValue::from(user_id.value()),
            aggregate_version: AggregateVersion::try_from(2).expect("version should be valid"),
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
    async fn removed_event_physically_deletes_the_fragment() {
        let user_fragment_writer = TestUserFragmentWriter::default();
        let deleted_user_ids = Arc::clone(&user_fragment_writer.deleted_user_ids);
        let projector = UserFragmentProjector::new(user_fragment_writer);
        let user_id = UserId::new();
        let event = removed_event_envelope(user_id);
        let event_context = MaterializationEventContext::from(&event);

        let invalidated_partitions = projector
            .project(&mut TestUow, event_context, &event)
            .await
            .expect("removed event should be projected");

        assert_eq!(*deleted_user_ids.lock().expect("lock"), vec![user_id]);

        assert_eq!(invalidated_partitions.len(), 1);
        assert_eq!(invalidated_partitions[0].key(), &user_id);
        let recorded_envelope = invalidation_envelope(&event, invalidated_partitions);
        assert_eq!(
            recorded_envelope.source_event_sequence,
            event.event_sequence
        );
        assert_eq!(recorded_envelope.invalidated_dependencies.len(), 1);
    }

    #[tokio::test]
    async fn activated_event_invalidates_the_written_fragment_partition() {
        let user_fragment_writer = TestUserFragmentWriter::default();
        let projector = UserFragmentProjector::new(user_fragment_writer);
        let user_id = UserId::new();
        let event = activated_event_envelope(user_id);
        let event_context = MaterializationEventContext::from(&event);

        let invalidated_partitions = projector
            .project(&mut TestUow, event_context, &event)
            .await
            .expect("activated event should be projected");

        assert_eq!(invalidated_partitions.len(), 1);
        assert_eq!(invalidated_partitions[0].key(), &user_id);
        let recorded_envelope = invalidation_envelope(&event, invalidated_partitions);
        assert_eq!(recorded_envelope.invalidated_dependencies.len(), 1);
    }
}
