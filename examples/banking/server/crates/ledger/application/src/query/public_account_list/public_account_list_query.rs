use appletheia::query;

use crate::read_model::{
    PublicAccountListCriteria, PublicAccountListCursor, PublicAccountListSortKey,
};
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

/// Query parameters for public account list reads.
#[query(name = "public_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListQuery {
    pub criteria: PublicAccountListCriteria,
    pub cursor_options: Option<CursorOptions<PublicAccountListSortKey, PublicAccountListCursor>>,
    pub limit: PageSize,
}
