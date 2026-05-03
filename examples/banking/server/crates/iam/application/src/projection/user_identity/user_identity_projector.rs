use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload};

use super::{UserIdentityProjectorError, UserIdentityProjectorSpec};
use crate::projection::{UserIdentityProjectionStore, UserIdentityProjectionUpsert};

/// Projects user identity events into normalized user identity projections.
pub struct UserIdentityProjector<VS>
where
    VS: UserIdentityProjectionStore,
{
    projection_store: VS,
}

impl<VS> UserIdentityProjector<VS>
where
    VS: UserIdentityProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for UserIdentityProjector<VS>
where
    VS: UserIdentityProjectionStore,
{
    type Spec = UserIdentityProjectorSpec;
    type Uow = VS::Uow;
    type Error = UserIdentityProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<User>()?;
        let user_id = domain_event.aggregate_id();

        match domain_event.payload() {
            UserEventPayload::IdentityLinked {
                provider,
                subject,
                email,
            } => {
                self.projection_store
                    .upsert(
                        uow,
                        UserIdentityProjectionUpsert {
                            user_id,
                            provider: provider.clone(),
                            subject: subject.clone(),
                            email: email.clone(),
                        },
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
                self.projection_store
                    .update_email(
                        uow,
                        provider.clone(),
                        subject.clone(),
                        email.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            UserEventPayload::Removed => {
                self.projection_store
                    .delete_by_user(uow, user_id, event.event_sequence, event.occurred_at)
                    .await?;
            }
            UserEventPayload::Registered { .. }
            | UserEventPayload::UsernameChanged { .. }
            | UserEventPayload::DisplayNameChanged { .. }
            | UserEventPayload::BioChanged { .. }
            | UserEventPayload::PictureChanged { .. }
            | UserEventPayload::Activated
            | UserEventPayload::Inactivated => {}
        }

        Ok(())
    }
}
