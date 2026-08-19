use banking_shared_kernel_application::read_model::SearchTerm;
use serde::{Deserialize, Serialize};

use crate::projection::MaterializedUserStatus;

/// Search criteria for public user list reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PublicUserListCriteria {
    pub username_contains: Vec<SearchTerm>,
    pub status_in: Option<Vec<MaterializedUserStatus>>,
}

impl Default for PublicUserListCriteria {
    fn default() -> Self {
        Self {
            username_contains: Vec::new(),
            status_in: Some(vec![MaterializedUserStatus::Active]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filters_to_active_users() {
        let criteria = PublicUserListCriteria::default();

        assert_eq!(
            criteria.status_in,
            Some(vec![MaterializedUserStatus::Active])
        );
    }

    #[test]
    fn deserialization_uses_the_active_default_when_status_is_omitted() {
        let criteria = serde_json::from_value::<PublicUserListCriteria>(serde_json::json!({
            "username_contains": ["alice"]
        }))
        .expect("criteria should deserialize");

        assert_eq!(
            criteria.status_in,
            Some(vec![MaterializedUserStatus::Active])
        );
    }

    #[test]
    fn deserialization_normalizes_username_search_terms() {
        let criteria = serde_json::from_value::<PublicUserListCriteria>(serde_json::json!({
            "username_contains": ["  ALI\u{3000}CE\u{00a0}_1 "]
        }))
        .expect("criteria should deserialize");

        assert_eq!(criteria.username_contains[0].as_ref(), "alice_1");
    }

    #[test]
    fn deserialization_rejects_an_empty_normalized_search_term() {
        let result = serde_json::from_value::<PublicUserListCriteria>(serde_json::json!({
            "username_contains": [" \u{3000}"]
        }));

        assert!(result.is_err());
    }
}
