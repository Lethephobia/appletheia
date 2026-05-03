use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipEventPayload};

use super::{OrganizationMembershipRoleProjectorError, OrganizationMembershipRoleProjectorSpec};
use crate::projection::{
    OrganizationMembershipRoleProjectionStore, OrganizationMembershipRoleProjectionUpsert,
};

/// Projects organization membership role events into normalized role projections.
pub struct OrganizationMembershipRoleProjector<VS>
where
    VS: OrganizationMembershipRoleProjectionStore,
{
    projection_store: VS,
}

impl<VS> OrganizationMembershipRoleProjector<VS>
where
    VS: OrganizationMembershipRoleProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for OrganizationMembershipRoleProjector<VS>
where
    VS: OrganizationMembershipRoleProjectionStore,
{
    type Spec = OrganizationMembershipRoleProjectorSpec;
    type Uow = VS::Uow;
    type Error = OrganizationMembershipRoleProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<OrganizationMembership>()?;
        let membership_id = domain_event.aggregate_id();

        match domain_event.payload() {
            OrganizationMembershipEventPayload::RoleGranted { role, .. } => {
                self.projection_store
                    .upsert(
                        uow,
                        OrganizationMembershipRoleProjectionUpsert {
                            organization_membership_id: membership_id,
                            role: *role,
                        },
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationMembershipEventPayload::RoleRevoked { role, .. } => {
                self.projection_store
                    .delete(
                        uow,
                        membership_id,
                        *role,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationMembershipEventPayload::Removed { .. } => {
                self.projection_store
                    .delete_by_membership(
                        uow,
                        membership_id,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            OrganizationMembershipEventPayload::Created { .. }
            | OrganizationMembershipEventPayload::Activated { .. }
            | OrganizationMembershipEventPayload::Inactivated { .. }
            | OrganizationMembershipEventPayload::RoleGrantRejected { .. }
            | OrganizationMembershipEventPayload::RoleRevokeRejected { .. }
            | OrganizationMembershipEventPayload::ActivateRejected { .. }
            | OrganizationMembershipEventPayload::DeactivateRejected { .. }
            | OrganizationMembershipEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
