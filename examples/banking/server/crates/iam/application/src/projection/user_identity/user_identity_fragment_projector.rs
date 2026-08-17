use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{MaterializationEventContext, ReadModelFragmentChange};
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
    ) -> Result<Vec<ReadModelFragmentChange<Self::Fragment>>, Self::Error> {
        let mut fragment_changes = Vec::new();
        let user_event = event.try_into_domain_event::<User>()?;
        let user_id = user_event.aggregate_id();

        match user_event.payload() {
            UserEventPayload::Registered {
                initial_identity: Some(identity),
                ..
            }
            | UserEventPayload::IdentityLinked { identity } => {
                if let Some(fragment) = self
                    .user_identity_fragment_writer
                    .upsert(
                        uow,
                        event_context,
                        UserIdentityFragmentUpsert {
                            user_id,
                            provider: identity.provider().clone(),
                            subject: identity.subject().clone(),
                            email: identity.email().cloned(),
                        },
                    )
                    .await?
                {
                    fragment_changes.push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                }
            }
            UserEventPayload::IdentityEmailChanged {
                provider,
                subject,
                email,
            } => {
                if let Some(fragment) = self
                    .user_identity_fragment_writer
                    .update_email(
                        uow,
                        event_context,
                        user_id,
                        provider.clone(),
                        subject.clone(),
                        email.clone(),
                    )
                    .await?
                {
                    fragment_changes.push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                }
            }
            UserEventPayload::Removed => {
                let removed_keys = self
                    .user_identity_fragment_writer
                    .delete_for_user(uow, event_context, user_id)
                    .await?;
                for key in removed_keys {
                    fragment_changes
                        .push(ReadModelFragmentChange::<UserIdentityFragment>::try_removed(&key)?);
                }
            }
            UserEventPayload::Registered {
                initial_identity: None,
                ..
            }
            | UserEventPayload::IdentityLinkRejected { .. }
            | UserEventPayload::IdentityEmailChangeRejected { .. }
            | UserEventPayload::UsernameChanged { .. }
            | UserEventPayload::UsernameChangeRejected { .. }
            | UserEventPayload::DisplayNameChanged { .. }
            | UserEventPayload::DisplayNameChangeRejected { .. }
            | UserEventPayload::BioChanged { .. }
            | UserEventPayload::BioChangeRejected { .. }
            | UserEventPayload::PictureChanged { .. }
            | UserEventPayload::PictureChangeRejected { .. }
            | UserEventPayload::OrganizationMembershipGranted { .. }
            | UserEventPayload::OrganizationMembershipGrantRejected { .. }
            | UserEventPayload::OrganizationMembershipRolesChanged { .. }
            | UserEventPayload::OrganizationMembershipRolesChangeRejected { .. }
            | UserEventPayload::OrganizationMembershipRemoved { .. }
            | UserEventPayload::OrganizationMembershipRemoveRejected { .. }
            | UserEventPayload::Activated
            | UserEventPayload::ActivateRejected { .. }
            | UserEventPayload::Deactivated
            | UserEventPayload::DeactivateRejected { .. }
            | UserEventPayload::RemoveRejected { .. } => {}
        }

        Ok(fragment_changes)
    }
}
