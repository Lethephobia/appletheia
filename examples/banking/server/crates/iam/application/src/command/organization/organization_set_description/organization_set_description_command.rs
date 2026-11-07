use appletheia::command;
use banking_iam_domain::{OrganizationDescription, OrganizationId};
use serde::{Deserialize, Serialize};

/// Changes an organization's description.
#[command(name = "organization_set_description")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationSetDescriptionCommand {
    pub organization_id: OrganizationId,
    pub description: Option<OrganizationDescription>,
}
