use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

/// The output returned after creating an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestCreateOutput {
    pub organization_join_request_id: OrganizationJoinRequestId,
}

impl OrganizationJoinRequestCreateOutput {
    /// Creates a new organization-join-request-create output.
    pub fn new(organization_join_request_id: OrganizationJoinRequestId) -> Self {
        Self {
            organization_join_request_id,
        }
    }
}
