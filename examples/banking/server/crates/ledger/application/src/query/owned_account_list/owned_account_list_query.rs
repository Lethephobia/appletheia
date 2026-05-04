use appletheia::query;
use banking_ledger_domain::account::AccountOwner;
use banking_ledger_domain::currency::CurrencyId;

use crate::pagination::{CursorOptions, PageLimit};
use crate::read_model::{
    OwnedAccountListItemCursor, OwnedAccountListItemSortKey, OwnedAccountListItemStatus,
};

/// Query parameters for account list reads.
#[query(name = "owned_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListQuery {
    pub owner: AccountOwner,
    pub currency_id: Option<CurrencyId>,
    pub status: Option<OwnedAccountListItemStatus>,
    pub cursor_options:
        Option<CursorOptions<OwnedAccountListItemSortKey, OwnedAccountListItemCursor>>,
    pub limit: PageLimit,
}
