use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationInvitationStatus,
};

use super::{OrganizationInvitationProjectorError, OrganizationInvitationProjectorSpec};
use crate::projection::{
    OrganizationInvitationProjectionStore, OrganizationInvitationProjectionUpsert,
};

/// Projects organization invitation events into normalized invitation projections.
pub struct OrganizationInvitationProjector<VS>
where
    VS: OrganizationInvitationProjectionStore,
{
    projection_store: VS,
}

impl<VS> OrganizationInvitationProjector<VS>
where
    VS: OrganizationInvitationProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for OrganizationInvitationProjector<VS>
where
    VS: OrganizationInvitationProjectionStore,
{
    type Spec = OrganizationInvitationProjectorSpec;
    type Uow = VS::Uow;
    type Error = OrganizationInvitationProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<OrganizationInvitation>()?;
        let invitation_id = domain_event.aggregate_id();

        match domain_event.payload() {
            OrganizationInvitationEventPayload::Issued {
                organization_id,
                invitee_id,
                issuer,
                expires_at,
                ..
            } => {
                self.projection_store
                    .upsert(
                        uow,
                        OrganizationInvitationProjectionUpsert {
                            id: invitation_id,
                            organization_id: *organization_id,
                            invitee_id: *invitee_id,
                            issuer: *issuer,
                            expires_at: *expires_at,
                            status: OrganizationInvitationStatus::Pending,
                        },
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::Accepted { .. } => {
                self.projection_store
                    .update_status(
                        uow,
                        invitation_id,
                        OrganizationInvitationStatus::Accepted,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::Declined { .. } => {
                self.projection_store
                    .update_status(
                        uow,
                        invitation_id,
                        OrganizationInvitationStatus::Declined,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::Canceled { .. } => {
                self.projection_store
                    .update_status(
                        uow,
                        invitation_id,
                        OrganizationInvitationStatus::Canceled,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationInvitationEventPayload::IssueRejected { .. }
            | OrganizationInvitationEventPayload::AcceptRejected { .. }
            | OrganizationInvitationEventPayload::DeclineRejected { .. }
            | OrganizationInvitationEventPayload::CancelRejected { .. } => {}
        }

        Ok(())
    }
}
