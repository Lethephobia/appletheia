use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload, UserStatus};

use super::{UserProjectorError, UserProjectorSpec};
use crate::view::{UserViewStore, UserViewUpsert};

/// Projects user events into normalized user views.
pub struct UserProjector<VS>
where
    VS: UserViewStore,
{
    view_store: VS,
}

impl<VS> UserProjector<VS>
where
    VS: UserViewStore,
{
    pub fn new(view_store: VS) -> Self {
        Self { view_store }
    }
}

impl<VS> Projector for UserProjector<VS>
where
    VS: UserViewStore,
{
    type Spec = UserProjectorSpec;
    type Uow = VS::Uow;
    type Error = UserProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<User>()?;
        let user_id = domain_event.aggregate_id();

        match domain_event.payload() {
            UserEventPayload::Registered { .. } => {
                self.view_store
                    .upsert(
                        uow,
                        UserViewUpsert {
                            id: user_id,
                            username: None,
                            display_name: None,
                            bio: None,
                            picture: None,
                            status: UserStatus::Active,
                        },
                        event.event_sequence,
                    )
                    .await?;
            }
            UserEventPayload::UsernameChanged { username } => {
                self.view_store
                    .update_username(uow, user_id, username.clone(), event.event_sequence)
                    .await?;
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                self.view_store
                    .update_display_name(uow, user_id, display_name.clone(), event.event_sequence)
                    .await?;
            }
            UserEventPayload::BioChanged { bio } => {
                self.view_store
                    .update_bio(uow, user_id, bio.clone(), event.event_sequence)
                    .await?;
            }
            UserEventPayload::PictureChanged { picture, .. } => {
                self.view_store
                    .update_picture(uow, user_id, picture.clone(), event.event_sequence)
                    .await?;
            }
            UserEventPayload::Activated => {
                self.view_store
                    .update_status(uow, user_id, UserStatus::Active, event.event_sequence)
                    .await?;
            }
            UserEventPayload::Inactivated => {
                self.view_store
                    .update_status(uow, user_id, UserStatus::Inactive, event.event_sequence)
                    .await?;
            }
            UserEventPayload::Removed => {
                self.view_store
                    .delete(uow, user_id, event.event_sequence)
                    .await?;
            }
            UserEventPayload::IdentityLinked { .. }
            | UserEventPayload::IdentityEmailChanged { .. } => {}
        }

        Ok(())
    }
}
