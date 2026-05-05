use appletheia::query;
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::CurrencyId;

use crate::pagination::{CursorOptions, PageLimit};
use crate::read_model::{
    OwnedAccountTransactionListItemCursor, OwnedAccountTransactionListItemSortKey,
    OwnedAccountTransactionListItemStatus,
};

/// Query parameters for owned account transaction list reads.
#[query(name = "owned_account_transaction_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListQuery {
    pub owner: AccountOwner,
    pub account_id: Option<AccountId>,
    pub currency_id: Option<CurrencyId>,
    pub status: Option<OwnedAccountTransactionListItemStatus>,
    pub cursor_options: Option<
        CursorOptions<
            OwnedAccountTransactionListItemSortKey,
            OwnedAccountTransactionListItemCursor,
        >,
    >,
    pub limit: PageLimit,
}
