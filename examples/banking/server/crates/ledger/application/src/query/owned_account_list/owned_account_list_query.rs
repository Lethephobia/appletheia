use appletheia::query;
use banking_ledger_domain::account::AccountOwner;

use crate::read_model::{
    OwnedAccountListCriteria, OwnedAccountListCursor, OwnedAccountListSortKey,
};
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

/// Query parameters for account list reads.
#[query(name = "owned_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListQuery {
    pub owner: AccountOwner,
    pub criteria: OwnedAccountListCriteria,
    pub cursor_options: Option<CursorOptions<OwnedAccountListSortKey, OwnedAccountListCursor>>,
    pub limit: PageSize,
}
