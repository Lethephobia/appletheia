use appletheia::command;
use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// Creates an organization join request.
#[command(name = "organization_join_request_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestCreateCommand {
    pub organization_id: OrganizationId,
}
