use appletheia::command;
use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

/// Activates an organization membership.
#[command(name = "organization_membership_activate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipActivateCommand {
    pub organization_membership_id: OrganizationMembershipId,
}
