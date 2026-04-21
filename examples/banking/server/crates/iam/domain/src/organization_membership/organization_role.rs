use serde::{Deserialize, Serialize};

/// Declares elevated organization roles granted through membership.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationRole {
    Admin,
    FinanceManager,
    Treasurer,
}

#[cfg(test)]
mod tests {
    use super::OrganizationRole;

    #[test]
    fn serializes_to_adjacently_tagged_json() {
        let value = serde_json::to_value(OrganizationRole::Admin).expect("serialize works");

        assert_eq!(value, serde_json::json!({ "type": "admin" }));
    }
}
