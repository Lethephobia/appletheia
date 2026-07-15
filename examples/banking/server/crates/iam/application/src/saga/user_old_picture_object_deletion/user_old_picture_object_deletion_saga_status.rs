use serde::{Deserialize, Serialize};

/// Describes progress for the user old picture object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserOldPictureObjectDeletionSagaStatus {
    DeleteRequested,
    Skipped,
}
