use serde::{Deserialize, Serialize};

/// Describes progress for the organization old picture object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationOldPictureObjectDeletionSagaStatus {
    DeleteRequested,
    Skipped,
}
