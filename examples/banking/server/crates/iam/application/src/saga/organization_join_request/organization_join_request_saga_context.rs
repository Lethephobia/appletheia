use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

/// Stores context for the organization join request saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestSagaContext {
    pub organization_join_request_id: OrganizationJoinRequestId,
}

impl OrganizationJoinRequestSagaContext {
    pub fn new(organization_join_request_id: OrganizationJoinRequestId) -> Self {
        Self {
            organization_join_request_id,
        }
    }
}
