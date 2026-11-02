use appletheia::command;
use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

/// Cancels an organization join request.
#[command(name = "organization_join_request_cancel")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestCancelCommand {
    pub organization_join_request_id: OrganizationJoinRequestId,
}
