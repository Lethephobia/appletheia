use appletheia::application::saga::SagaState;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Stores state for the user old picture object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOldPictureObjectDeletionSagaState {
    pub user_id: UserId,
}

impl UserOldPictureObjectDeletionSagaState {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }
}

impl SagaState for UserOldPictureObjectDeletionSagaState {}
