use std::cmp::Ordering;

use appletheia::application::read_model::list::ReadModelListSortKey;
use appletheia::domain::AggregateId;
use serde::{Deserialize, Serialize};

use crate::projection::PublicUserListItemPart;

use super::PublicUserListCursor;

/// Sort key for public user list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicUserListSortKey {
    CreatedAt,
    UserId,
}

impl ReadModelListSortKey for PublicUserListSortKey {
    type Candidate = PublicUserListItemPart;
    type Cursor = PublicUserListCursor;

    fn cursor(&self, candidate: &Self::Candidate) -> Self::Cursor {
        PublicUserListCursor {
            created_at: candidate.created_at,
            user_id: candidate.user_id,
        }
    }

    fn compare_to_cursor(&self, candidate: &Self::Candidate, cursor: &Self::Cursor) -> Ordering {
        match self {
            Self::CreatedAt => (candidate.created_at, candidate.user_id.value())
                .cmp(&(cursor.created_at, cursor.user_id.value())),
            Self::UserId => candidate.user_id.value().cmp(&cursor.user_id.value()),
        }
    }
}

impl PublicUserListSortKey {
    /// Creates a cursor from the materialized public-user list item.
    pub fn cursor_for_item(&self, item: &PublicUserListItemPart) -> PublicUserListCursor {
        PublicUserListCursor {
            created_at: item.created_at,
            user_id: item.user_id,
        }
    }
}
