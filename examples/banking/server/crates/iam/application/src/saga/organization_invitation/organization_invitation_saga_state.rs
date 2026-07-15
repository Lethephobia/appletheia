use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

use super::OrganizationInvitationSagaStatus;

/// Stores the progress of the organization invitation saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationSagaState {
    pub organization_invitation_id: OrganizationInvitationId,
    pub status: OrganizationInvitationSagaStatus,
}

impl OrganizationInvitationSagaState {
    pub fn new(organization_invitation_id: OrganizationInvitationId) -> Self {
        Self {
            organization_invitation_id,
            status: OrganizationInvitationSagaStatus::MembershipGrantRequested,
        }
    }
}

impl SagaState for OrganizationInvitationSagaState {}
