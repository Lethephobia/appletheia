use appletheia::query;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use crate::read_model::{PublicUserListCriteria, PublicUserListCursor, PublicUserListSortKey};

/// Query parameters for public user list reads.
#[query(name = "public_user_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicUserListQuery {
    pub criteria: PublicUserListCriteria,
    pub cursor_options: Option<CursorOptions<PublicUserListSortKey, PublicUserListCursor>>,
    pub limit: PageSize,
}
