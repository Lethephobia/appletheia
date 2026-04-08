use appletheia::command;
use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// Removes the specified organization.
#[command(name = "organization_remove")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationRemoveCommand {
    pub organization_id: OrganizationId,
}
