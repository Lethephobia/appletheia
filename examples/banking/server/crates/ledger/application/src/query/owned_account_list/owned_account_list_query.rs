use appletheia::query;
use banking_ledger_domain::account::AccountOwner;

use crate::pagination::{CursorOptions, PageLimit};
use crate::read_model::{
    OwnedAccountListItemCriteria, OwnedAccountListItemCursor, OwnedAccountListItemSortKey,
};

/// Query parameters for account list reads.
#[query(name = "owned_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListQuery {
    pub owner: AccountOwner,
    pub criteria: OwnedAccountListItemCriteria,
    pub cursor_options:
        Option<CursorOptions<OwnedAccountListItemSortKey, OwnedAccountListItemCursor>>,
    pub limit: PageLimit,
}
