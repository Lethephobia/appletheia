use appletheia::command;
use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// Deletes the specified organization.
#[command(name = "organization_delete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDeleteCommand {
    pub organization_id: OrganizationId,
}
