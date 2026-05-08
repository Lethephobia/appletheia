use appletheia::query;

use crate::pagination::{CursorOptions, PageSize};
use crate::read_model::{
    PublicAccountListItemCriteria, PublicAccountListItemCursor, PublicAccountListItemSortKey,
};

/// Query parameters for public account list reads.
#[query(name = "public_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListQuery {
    pub criteria: PublicAccountListItemCriteria,
    pub cursor_options:
        Option<CursorOptions<PublicAccountListItemSortKey, PublicAccountListItemCursor>>,
    pub limit: PageSize,
}
