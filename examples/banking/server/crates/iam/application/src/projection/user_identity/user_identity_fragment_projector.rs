use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::MaterializationEventContext;
use banking_iam_domain::{User, UserEventPayload};

use crate::projection::{
    UserIdentityFragment, UserIdentityFragmentUpsert, UserIdentityFragmentWriter,
};

use super::{UserIdentityFragmentProjectorError, UserIdentityFragmentProjectorSpec};

/// Projects user events into user identity fragments.
pub struct UserIdentityFragmentProjector<W>
where
    W: UserIdentityFragmentWriter,
{
    user_identity_fragment_writer: W,
}

impl<W> UserIdentityFragmentProjector<W>
where
    W: UserIdentityFragmentWriter,
{
    pub fn new(user_identity_fragment_writer: W) -> Self {
        Self {
            user_identity_fragment_writer,
        }
    }
}

impl<W> Projector for UserIdentityFragmentProjector<W>
where
    W: UserIdentityFragmentWriter,
{
    type Spec = UserIdentityFragmentProjectorSpec;
    type Fragment = UserIdentityFragment;
    type Uow = W::Uow;
    type Error = UserIdentityFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let user_event = event.try_to_domain_event::<User>()?;
        let user_id = user_event.aggregate_id();

        match user_event.payload() {
            UserEventPayload::IdentityLinked {
                provider,
                subject,
                email,
            } => {
                self.user_identity_fragment_writer
                    .upsert(
                        uow,
                        event_context,
                        UserIdentityFragmentUpsert {
                            user_id,
                            provider: provider.clone(),
                            subject: subject.clone(),
                            email: email.clone(),
                        },
                    )
                    .await?;
            }
            UserEventPayload::IdentityEmailChanged {
                provider,
                subject,
                email,
            } => {
                self.user_identity_fragment_writer
                    .update_email(
                        uow,
                        event_context,
                        user_id,
                        provider.clone(),
                        subject.clone(),
                        email.clone(),
                    )
                    .await?;
            }
            UserEventPayload::Removed => {
                self.user_identity_fragment_writer
                    .delete_for_user(uow, event_context, user_id)
                    .await?;
            }
            UserEventPayload::Registered
            | UserEventPayload::UsernameChanged { .. }
            | UserEventPayload::DisplayNameChanged { .. }
            | UserEventPayload::BioChanged { .. }
            | UserEventPayload::PictureChanged { .. }
            | UserEventPayload::Activated
            | UserEventPayload::Deactivated => {}
        }

        Ok(())
    }
}
