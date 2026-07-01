use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

use super::OrganizationOldPictureObjectDeletionSagaStatus;

/// Stores state for the organization old picture object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationOldPictureObjectDeletionSagaState {
    pub organization_id: OrganizationId,
    pub status: OrganizationOldPictureObjectDeletionSagaStatus,
}

impl OrganizationOldPictureObjectDeletionSagaState {
    pub fn new(organization_id: OrganizationId) -> Self {
        Self {
            organization_id,
            status: OrganizationOldPictureObjectDeletionSagaStatus::DeleteRequested,
        }
    }
}

impl SagaState for OrganizationOldPictureObjectDeletionSagaState {}
