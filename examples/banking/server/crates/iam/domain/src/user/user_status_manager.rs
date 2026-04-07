use serde::{Deserialize, Serialize};

use super::UserId;

/// Identifies who manages a user's status.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserStatusManager {
    User(UserId),
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::UserStatusManager;
    use crate::UserId;

    #[test]
    fn user_variant_preserves_user_id() {
        let user_id = UserId::try_from_uuid(Uuid::now_v7()).expect("user id should be accepted");
        let status_manager = UserStatusManager::User(user_id);

        assert_eq!(status_manager, UserStatusManager::User(user_id));
    }
}
