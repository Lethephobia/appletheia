use appletheia::application::saga::SagaState;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Stores state for the user picture saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureSagaState {
    pub user_id: UserId,
}

impl UserPictureSagaState {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }
}

impl SagaState for UserPictureSagaState {}
