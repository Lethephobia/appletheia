use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload};

use crate::read_model::{UserPublicProfileStatus, UserPublicProfileWriter};

use super::{UserPublicProfileProjectorError, UserPublicProfileProjectorSpec};

/// Projects user events into the public profile read model.
pub struct UserPublicProfileProjector<W>
where
    W: UserPublicProfileWriter,
{
    writer: W,
}

impl<W> UserPublicProfileProjector<W>
where
    W: UserPublicProfileWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for UserPublicProfileProjector<W>
where
    W: UserPublicProfileWriter,
{
    type Spec = UserPublicProfileProjectorSpec;
    type Uow = W::Uow;
    type Error = UserPublicProfileProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<User>()?;
        let user_id = domain_event.aggregate_id();

        match domain_event.payload() {
            UserEventPayload::Registered { .. } => {
                self.writer
                    .upsert_user(
                        uow,
                        user_id,
                        UserPublicProfileStatus::Active,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::UsernameChanged { username } => {
                self.writer
                    .update_username(
                        uow,
                        user_id,
                        username.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                self.writer
                    .update_display_name(
                        uow,
                        user_id,
                        display_name.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::BioChanged { bio } => {
                self.writer
                    .update_bio(
                        uow,
                        user_id,
                        bio.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::PictureChanged { picture, .. } => {
                self.writer
                    .update_picture(
                        uow,
                        user_id,
                        picture.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::Activated => {
                self.writer
                    .update_status(
                        uow,
                        user_id,
                        UserPublicProfileStatus::Active,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::Inactivated => {
                self.writer
                    .update_status(
                        uow,
                        user_id,
                        UserPublicProfileStatus::Inactive,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::Removed => {
                self.writer
                    .delete_user(uow, user_id, event.event_sequence, event.occurred_at)
                    .await?;
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
            | UserEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
