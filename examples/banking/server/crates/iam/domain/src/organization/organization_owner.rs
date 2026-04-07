use serde::{Deserialize, Serialize};

use crate::UserId;

/// Identifies who owns an organization.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationOwner {
    User(UserId),
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::OrganizationOwner;
    use crate::UserId;

    #[test]
    fn user_variant_preserves_user_id() {
        let user_id = UserId::try_from_uuid(Uuid::now_v7()).expect("user id should be accepted");
        let owner = OrganizationOwner::User(user_id);

        assert_eq!(owner, OrganizationOwner::User(user_id));
    }
}
