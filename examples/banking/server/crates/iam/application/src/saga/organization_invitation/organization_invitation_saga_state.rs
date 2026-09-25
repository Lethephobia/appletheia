use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// Stores data needed by the organization invitation saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationSagaState {
    pub organization_invitation_id: OrganizationInvitationId,
}

impl OrganizationInvitationSagaState {
    pub fn new(organization_invitation_id: OrganizationInvitationId) -> Self {
        Self {
            organization_invitation_id,
        }
    }
}

impl SagaState for OrganizationInvitationSagaState {}
