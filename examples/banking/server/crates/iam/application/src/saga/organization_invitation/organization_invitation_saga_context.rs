use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// Stores context for the organization invitation saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationSagaContext {
    pub organization_invitation_id: OrganizationInvitationId,
}

impl OrganizationInvitationSagaContext {
    pub fn new(organization_invitation_id: OrganizationInvitationId) -> Self {
        Self {
            organization_invitation_id,
        }
    }
}
