use serde::{Deserialize, Serialize};

use super::PublicUserListItem;

use super::PublicUserListCursor;

/// Sort key for public user list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicUserListSortKey {
    CreatedAt,
    UserId,
}

impl PublicUserListSortKey {
    /// Creates a cursor from the materialized public-user list item.
    pub fn cursor_for_item(&self, item: &PublicUserListItem) -> PublicUserListCursor {
        PublicUserListCursor {
            created_at: item.created_at,
            user_id: item.user_id,
        }
    }
}
