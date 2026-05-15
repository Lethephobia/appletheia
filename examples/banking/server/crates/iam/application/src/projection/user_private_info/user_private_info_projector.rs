use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload};

use crate::read_model::{
    UserPrivateInfoIdentityUpsert, UserPrivateInfoStatus, UserPrivateInfoUserUpsert,
    UserPrivateInfoWriter,
};

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
            UserEventPayload::Registered {
                identities,
                username,
                display_name,
                bio,
                picture,
                status,
                ..
            } => {
                self.writer
                    .upsert_user(
                        uow,
                        UserPrivateInfoUserUpsert {
                            id: user_id,
                            username: username.clone(),
                            display_name: display_name.clone(),
                            bio: bio.clone(),
                            picture: picture.clone(),
                            status: UserPrivateInfoStatus::try_from(*status)?,
                            event_id: event.event_id,
                            event_sequence: event.event_sequence,
                            occurred_at: event.occurred_at,
                        },
                    )
                    .await?;
                for identity in identities {
                    self.writer
                        .upsert_identity(
                            uow,
                            UserPrivateInfoIdentityUpsert {
                                user_id,
                                provider: identity.provider().clone(),
                                subject: identity.subject().clone(),
                                email: identity.email().cloned(),
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
            }
            UserEventPayload::IdentityLinked {
                provider,
                subject,
                email,
            } => {
                self.writer
                    .upsert_identity(
                        uow,
                        UserPrivateInfoIdentityUpsert {
                            user_id,
                            provider: provider.clone(),
                            subject: subject.clone(),
                            email: email.clone(),
                            event_id: event.event_id,
                            event_sequence: event.event_sequence,
                            occurred_at: event.occurred_at,
                        },
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
                        UserPrivateInfoIdentityUpsert {
                            user_id,
                            provider: provider.clone(),
                            subject: subject.clone(),
                            email: email.clone(),
                            event_id: event.event_id,
                            event_sequence: event.event_sequence,
                            occurred_at: event.occurred_at,
                        },
                    )
                    .await?;
            }
            UserEventPayload::UsernameChanged { username } => {
                self.writer
                    .update_username(
                        uow,
                        user_id,
                        username.clone(),
                        event.event_id,
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
                        event.event_id,
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
                        event.event_id,
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
                        event.event_id,
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
                        event.event_id,
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
                        event.event_id,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::Removed => {
                self.writer
                    .delete_user(
                        uow,
                        user_id,
                        event.event_id,
                        event.event_sequence,
                        event.occurred_at,
                    )
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
