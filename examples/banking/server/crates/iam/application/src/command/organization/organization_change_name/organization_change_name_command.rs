use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationName};
use serde::{Deserialize, Serialize};

/// Changes an organization's name.
#[command(name = "organization_change_name")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeNameCommand {
    pub organization_id: OrganizationId,
    pub name: OrganizationName,
}
