use appletheia::command;
use banking_iam_domain::{OrganizationDescription, OrganizationId};
use serde::{Deserialize, Serialize};

/// Changes an organization's description.
#[command(name = "organization_description_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDescriptionChangeCommand {
    pub organization_id: OrganizationId,
    pub description: Option<OrganizationDescription>,
}
