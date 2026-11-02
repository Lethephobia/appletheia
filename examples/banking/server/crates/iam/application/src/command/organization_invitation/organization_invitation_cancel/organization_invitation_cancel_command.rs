use appletheia::command;
use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// Cancels an organization invitation.
#[command(name = "organization_invitation_cancel")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationCancelCommand {
    pub organization_invitation_id: OrganizationInvitationId,
}
