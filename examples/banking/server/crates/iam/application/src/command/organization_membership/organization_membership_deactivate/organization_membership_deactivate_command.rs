use appletheia::command;
use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

/// Deactivates an organization membership.
#[command(name = "organization_membership_deactivate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipDeactivateCommand {
    pub organization_membership_id: OrganizationMembershipId,
}
