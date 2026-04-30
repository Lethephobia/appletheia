use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// Stores state for the organization picture saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPictureSagaState {
    pub organization_id: OrganizationId,
}

impl OrganizationPictureSagaState {
    pub fn new(organization_id: OrganizationId) -> Self {
        Self { organization_id }
    }
}

impl SagaState for OrganizationPictureSagaState {}
