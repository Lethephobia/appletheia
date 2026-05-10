use appletheia::query;
use banking_ledger_domain::account::AccountOwner;

use crate::read_model::{CursorOptions, PageSize};
use crate::read_model::{
    OwnedAccountListCriteria, OwnedAccountListCursor, OwnedAccountListSortKey,
};

/// Query parameters for account list reads.
#[query(name = "owned_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListQuery {
    pub owner: AccountOwner,
    pub criteria: OwnedAccountListCriteria,
    pub cursor_options: Option<CursorOptions<OwnedAccountListSortKey, OwnedAccountListCursor>>,
    pub limit: PageSize,
}
