use appletheia::query;
use banking_ledger_domain::account::{AccountOwner, AccountStatus};
use banking_ledger_domain::currency::CurrencyId;

use crate::query::{CursorOptions, PageLimit};

use super::{OwnedAccountListCursor, OwnedAccountListSortKey};

/// Query parameters for account list reads.
#[query(name = "owned_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListQuery {
    pub owner: AccountOwner,
    pub currency_id: Option<CurrencyId>,
    pub status: Option<AccountStatus>,
    pub cursor_options: Option<CursorOptions<OwnedAccountListSortKey, OwnedAccountListCursor>>,
    pub limit: PageLimit,
}
