use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationJoinRequestId;
use serde::{Deserialize, Serialize};

/// Stores the progress of the organization join request saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestSagaState {
    pub organization_join_request_id: Option<OrganizationJoinRequestId>,
}

impl SagaState for OrganizationJoinRequestSagaState {}
