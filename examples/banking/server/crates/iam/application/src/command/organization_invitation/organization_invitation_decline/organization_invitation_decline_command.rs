use appletheia::command;
use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// Declines an organization invitation.
#[command(name = "organization_invitation_decline")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationDeclineCommand {
    pub organization_invitation_id: OrganizationInvitationId,
}
