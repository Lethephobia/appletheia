use appletheia::command;
use banking_iam_domain::{OrganizationHandle, OrganizationId};
use serde::{Deserialize, Serialize};

/// Changes an organization's handle.
#[command(name = "organization_change_handle")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeHandleCommand {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
}
