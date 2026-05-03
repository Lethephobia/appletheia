use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    OrganizationMembership, OrganizationMembershipEventPayload, OrganizationMembershipStatus,
};

use super::{OrganizationMembershipProjectorError, OrganizationMembershipProjectorSpec};
use crate::projection::{
    OrganizationMembershipProjectionStore, OrganizationMembershipProjectionUpsert,
};

/// Projects organization membership events into normalized membership projections.
pub struct OrganizationMembershipProjector<VS>
where
    VS: OrganizationMembershipProjectionStore,
{
    projection_store: VS,
}

impl<VS> OrganizationMembershipProjector<VS>
where
    VS: OrganizationMembershipProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for OrganizationMembershipProjector<VS>
where
    VS: OrganizationMembershipProjectionStore,
{
    type Spec = OrganizationMembershipProjectorSpec;
    type Uow = VS::Uow;
    type Error = OrganizationMembershipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<OrganizationMembership>()?;
        let membership_id = domain_event.aggregate_id();

        match domain_event.payload() {
            OrganizationMembershipEventPayload::Created {
                organization_id,
                user_id,
                ..
            } => {
                self.projection_store
                    .upsert(
                        uow,
                        OrganizationMembershipProjectionUpsert {
                            id: membership_id,
                            organization_id: *organization_id,
                            user_id: *user_id,
                            status: OrganizationMembershipStatus::Active,
                        },
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationMembershipEventPayload::Activated { .. } => {
                self.projection_store
                    .update_status(
                        uow,
                        membership_id,
                        OrganizationMembershipStatus::Active,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationMembershipEventPayload::Inactivated { .. } => {
                self.projection_store
                    .update_status(
                        uow,
                        membership_id,
                        OrganizationMembershipStatus::Inactive,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationMembershipEventPayload::Removed { .. } => {
                self.projection_store
                    .delete(uow, membership_id, event.event_sequence, event.occurred_at)
                    .await?;
            }
            OrganizationMembershipEventPayload::RoleGranted { .. }
            | OrganizationMembershipEventPayload::RoleRevoked { .. }
            | OrganizationMembershipEventPayload::RoleGrantRejected { .. }
            | OrganizationMembershipEventPayload::RoleRevokeRejected { .. }
            | OrganizationMembershipEventPayload::ActivateRejected { .. }
            | OrganizationMembershipEventPayload::DeactivateRejected { .. }
            | OrganizationMembershipEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
