use appletheia::query;

use crate::read_model::{CursorOptions, PageSize};
use crate::read_model::{
    PublicAccountListCriteria, PublicAccountListCursor, PublicAccountListSortKey,
};

/// Query parameters for public account list reads.
#[query(name = "public_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListQuery {
    pub criteria: PublicAccountListCriteria,
    pub cursor_options: Option<CursorOptions<PublicAccountListSortKey, PublicAccountListCursor>>,
    pub limit: PageSize,
}
