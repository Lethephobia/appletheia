use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

/// Identifies the owner of an `Account`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum AccountOwner {
    User(UserId),
}

impl AccountOwner {
    /// Creates a user-owned account owner.
    pub fn user(user_id: UserId) -> Self {
        Self::User(user_id)
    }

    /// Returns the user id for a user-owned account.
    pub fn user_id(&self) -> &UserId {
        match self {
            Self::User(user_id) => user_id,
        }
    }
}

impl From<UserId> for AccountOwner {
    fn from(value: UserId) -> Self {
        Self::User(value)
    }
}

#[cfg(test)]
mod tests {
    use banking_iam_domain::UserId;

    use super::AccountOwner;

    #[test]
    fn serializes_to_json() {
        let owner = AccountOwner::User(UserId::new());
        let value = serde_json::to_value(&owner).expect("owner should serialize");

        assert_eq!(value["type"], serde_json::json!("user"));
    }
}
