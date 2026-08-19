use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::query;

use crate::read_model::{PublicUserListCriteria, PublicUserListCursor, PublicUserListSortKey};

/// Query parameters for public user list reads.
#[query(name = "public_user_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicUserListQuery {
    pub criteria: PublicUserListCriteria,
    pub sort: Sort<PublicUserListSortKey>,
    pub page: CursorWindow<PublicUserListCursor>,
}
