use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload};

use crate::read_model::{UserPrivateInfoStatus, UserPrivateInfoWriter};

use super::{UserPrivateInfoProjectorError, UserPrivateInfoProjectorSpec};

/// Projects user events into the private information read model.
pub struct UserPrivateInfoProjector<W>
where
    W: UserPrivateInfoWriter,
{
    writer: W,
}

impl<W> UserPrivateInfoProjector<W>
where
    W: UserPrivateInfoWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for UserPrivateInfoProjector<W>
where
    W: UserPrivateInfoWriter,
{
    type Spec = UserPrivateInfoProjectorSpec;
    type Uow = W::Uow;
    type Error = UserPrivateInfoProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<User>()?;
        let user_id = domain_event.aggregate_id();

        match domain_event.payload() {
            UserEventPayload::Registered { .. } => {
                self.writer
                    .upsert_user(
                        uow,
                        user_id,
                        UserPrivateInfoStatus::Active,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::IdentityLinked {
                provider,
                subject,
                email,
            } => {
                self.writer
                    .upsert_identity(
                        uow,
                        user_id,
                        provider.clone(),
                        subject.clone(),
                        email.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::IdentityEmailChanged {
                provider,
                subject,
                email,
            } => {
                self.writer
                    .update_identity_email(
                        uow,
                        user_id,
                        provider.clone(),
                        subject.clone(),
                        email.clone(),
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
                        UserPrivateInfoStatus::Active,
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
                        UserPrivateInfoStatus::Inactive,
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
            UserEventPayload::IdentityLinkRejected { .. }
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
