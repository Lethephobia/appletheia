use appletheia::command;
use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

/// Approves an organization join request.
#[command(name = "organization_join_request_approve")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestApproveCommand {
    pub organization_join_request_id: OrganizationJoinRequestId,
}
