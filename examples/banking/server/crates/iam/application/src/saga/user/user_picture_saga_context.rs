use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Stores context for the user picture saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureSagaContext {
    pub user_id: UserId,
}

impl UserPictureSagaContext {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }
}
