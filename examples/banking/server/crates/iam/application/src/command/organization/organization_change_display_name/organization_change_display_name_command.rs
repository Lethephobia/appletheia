use appletheia::command;
use banking_iam_domain::{OrganizationDisplayName, OrganizationId};
use serde::{Deserialize, Serialize};

/// Changes an organization's display name.
#[command(name = "organization_change_display_name")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeDisplayNameCommand {
    pub organization_id: OrganizationId,
    pub display_name: OrganizationDisplayName,
}
