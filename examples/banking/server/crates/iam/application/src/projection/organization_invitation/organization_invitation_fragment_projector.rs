use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::MaterializationEventContext;
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationInvitationStatus,
};

use crate::projection::{
    OrganizationInvitationFragment, OrganizationInvitationFragmentUpsert,
    OrganizationInvitationFragmentWriter,
};

use super::{
    OrganizationInvitationFragmentProjectorError, OrganizationInvitationFragmentProjectorSpec,
};

/// Projects invitation events into organization invitation fragments.
pub struct OrganizationInvitationFragmentProjector<W>
where
    W: OrganizationInvitationFragmentWriter,
{
    organization_invitation_fragment_writer: W,
}

impl<W> OrganizationInvitationFragmentProjector<W>
where
    W: OrganizationInvitationFragmentWriter,
{
    pub fn new(organization_invitation_fragment_writer: W) -> Self {
        Self {
            organization_invitation_fragment_writer,
        }
    }
}

impl<W> Projector for OrganizationInvitationFragmentProjector<W>
where
    W: OrganizationInvitationFragmentWriter,
{
    type Spec = OrganizationInvitationFragmentProjectorSpec;
    type Fragment = OrganizationInvitationFragment;
    type Uow = W::Uow;
    type Error = OrganizationInvitationFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let invitation_event = event.try_to_domain_event::<OrganizationInvitation>()?;
        let invitation_id = invitation_event.aggregate_id();

        match invitation_event.payload() {
            OrganizationInvitationEventPayload::Issued {
                organization_id,
                invitee_id,
                roles,
                issuer,
                expires_at,
            } => {
                self.organization_invitation_fragment_writer
                    .upsert(
                        uow,
                        event_context,
                        OrganizationInvitationFragmentUpsert {
                            invitation_id,
                            organization_id: *organization_id,
                            invitee_user_id: *invitee_id,
                            roles: roles.clone(),
                            issuer: *issuer,
                            expires_at: *expires_at,
                            status: OrganizationInvitationStatus::Pending,
                        },
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::Accepted { .. } => {
                self.organization_invitation_fragment_writer
                    .update_status(
                        uow,
                        event_context,
                        invitation_id,
                        OrganizationInvitationStatus::Accepted,
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::Declined { .. } => {
                self.organization_invitation_fragment_writer
                    .update_status(
                        uow,
                        event_context,
                        invitation_id,
                        OrganizationInvitationStatus::Declined,
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::Canceled { .. } => {
                self.organization_invitation_fragment_writer
                    .update_status(
                        uow,
                        event_context,
                        invitation_id,
                        OrganizationInvitationStatus::Canceled,
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
