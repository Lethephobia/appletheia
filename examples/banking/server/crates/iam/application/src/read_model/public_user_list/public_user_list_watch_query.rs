use appletheia::application::read_model::list::ReadModelListQuery;
use appletheia::application::read_model::pagination::{Sort, SortDirection};
use serde::{Deserialize, Serialize};

use super::{PublicUserListCriteria, PublicUserListSortKey};

/// Identifies criteria and ordering for a watched public user list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicUserListWatchQuery {
    pub criteria: PublicUserListCriteria,
    pub sort: Sort<PublicUserListSortKey>,
}

impl Default for PublicUserListWatchQuery {
    fn default() -> Self {
        Self {
            criteria: PublicUserListCriteria::default(),
            sort: Sort {
                key: PublicUserListSortKey::CreatedAt,
                direction: SortDirection::Desc,
            },
        }
    }
}

impl ReadModelListQuery for PublicUserListWatchQuery {
    type Criteria = PublicUserListCriteria;
    type SortKey = PublicUserListSortKey;

    fn criteria(&self) -> &Self::Criteria {
        &self.criteria
    }

    fn sort(&self) -> &Sort<Self::SortKey> {
        &self.sort
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::PublicUserListWatchQuery;

    #[test]
    fn default_query_serializes_the_standard_sort_shape() {
        let serialized = serde_json::to_value(PublicUserListWatchQuery::default())
            .expect("watch query should serialize");

        assert_eq!(serialized["sort"]["key"], json!("created_at"));
        assert_eq!(serialized["sort"]["direction"], json!("desc"));
    }
}
