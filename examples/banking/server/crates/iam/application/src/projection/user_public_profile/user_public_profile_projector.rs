use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload};

use crate::read_model::{
    ReadModelEventContext, UserPublicProfileStatus, UserPublicProfileUserUpsert,
    UserPublicProfileWriter,
};

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
        let event_context = ReadModelEventContext::from(event);
        let domain_event = event.try_into_domain_event::<User>()?;
        let user_id = domain_event.aggregate_id();

        match domain_event.payload() {
            UserEventPayload::Registered { .. } => {
                self.writer
                    .upsert_user(
                        uow,
                        event_context,
                        UserPublicProfileUserUpsert {
                            id: user_id,
                            username: None,
                            display_name: None,
                            bio: None,
                            picture: None,
                            status: UserPublicProfileStatus::Active,
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
            UserEventPayload::BioChanged { bio } => {
                self.writer
                    .update_bio(uow, event_context, user_id, bio.clone())
                    .await?;
            }
            UserEventPayload::PictureChanged { picture, .. } => {
                self.writer
                    .update_picture(uow, event_context, user_id, picture.clone())
                    .await?;
            }
            UserEventPayload::Activated => {
                self.writer
                    .update_status(uow, event_context, user_id, UserPublicProfileStatus::Active)
                    .await?;
            }
            UserEventPayload::Deactivated => {
                self.writer
                    .update_status(
                        uow,
                        event_context,
                        user_id,
                        UserPublicProfileStatus::Inactive,
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
