use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

/// Stores the progress of the organization join request saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestSagaState {
    pub organization_join_request_id: OrganizationJoinRequestId,
}

impl OrganizationJoinRequestSagaState {
    pub fn new(organization_join_request_id: OrganizationJoinRequestId) -> Self {
        Self {
            organization_join_request_id,
        }
    }
}

impl SagaState for OrganizationJoinRequestSagaState {}
