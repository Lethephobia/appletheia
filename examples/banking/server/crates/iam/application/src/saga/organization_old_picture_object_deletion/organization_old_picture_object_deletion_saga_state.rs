use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// Stores state for the organization old picture object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationOldPictureObjectDeletionSagaState {
    pub organization_id: OrganizationId,
}

impl OrganizationOldPictureObjectDeletionSagaState {
    pub fn new(organization_id: OrganizationId) -> Self {
        Self { organization_id }
    }
}

impl SagaState for OrganizationOldPictureObjectDeletionSagaState {}
