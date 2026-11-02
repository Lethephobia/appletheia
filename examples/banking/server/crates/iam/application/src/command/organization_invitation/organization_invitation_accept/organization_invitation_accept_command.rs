use appletheia::command;
use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// Accepts an organization invitation.
#[command(name = "organization_invitation_accept")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationAcceptCommand {
    pub organization_invitation_id: OrganizationInvitationId,
}
