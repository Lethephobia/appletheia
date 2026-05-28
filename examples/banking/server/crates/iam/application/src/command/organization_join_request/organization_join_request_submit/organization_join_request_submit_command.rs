use appletheia::command;
use banking_iam_domain::{OrganizationId, UserId};
use serde::{Deserialize, Serialize};

/// Submits an organization join request.
#[command(name = "organization_join_request_submit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestSubmitCommand {
    pub organization_id: OrganizationId,
    pub requester_id: UserId,
}
