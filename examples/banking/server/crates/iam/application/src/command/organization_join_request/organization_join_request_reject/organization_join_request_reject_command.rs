use appletheia::command;
use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

/// Rejects an organization join request.
#[command(name = "organization_join_request_reject")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestRejectCommand {
    pub organization_join_request_id: OrganizationJoinRequestId,
}
