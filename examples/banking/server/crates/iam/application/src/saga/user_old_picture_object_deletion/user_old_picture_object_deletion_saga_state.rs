use appletheia::application::saga::SagaState;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

use super::UserOldPictureObjectDeletionSagaStatus;

/// Stores state for the user old picture object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOldPictureObjectDeletionSagaState {
    pub user_id: UserId,
    pub status: UserOldPictureObjectDeletionSagaStatus,
}

impl UserOldPictureObjectDeletionSagaState {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            status: UserOldPictureObjectDeletionSagaStatus::DeleteRequested,
        }
    }
}

impl SagaState for UserOldPictureObjectDeletionSagaState {}
