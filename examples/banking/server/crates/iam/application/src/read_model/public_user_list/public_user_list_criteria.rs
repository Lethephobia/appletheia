use appletheia::application::read_model::list::ReadModelListCriteria;
use banking_shared_kernel_application::read_model::SearchTerm;
use serde::{Deserialize, Serialize};

use crate::projection::{MaterializedUserStatus, PublicUserListItemPart};

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

impl ReadModelListCriteria for PublicUserListCriteria {
    type Candidate = PublicUserListItemPart;

    fn matches(&self, candidate: &Self::Candidate) -> bool {
        if !self.username_contains.is_empty() {
            let username = match &candidate.username {
                Some(username) => username,
                None => return false,
            };

            if self
                .username_contains
                .iter()
                .any(|term| !term.matches(username))
            {
                return false;
            }
        }

        if let Some(status_in) = &self.status_in
            && !status_in.contains(&candidate.status)
        {
            return false;
        }

        true
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
