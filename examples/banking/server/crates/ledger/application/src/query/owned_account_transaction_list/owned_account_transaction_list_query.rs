use appletheia::query;
use banking_ledger_domain::account::AccountOwner;

use crate::pagination::{CursorOptions, PageLimit};
use crate::read_model::{
    OwnedAccountTransactionListItemCriteria, OwnedAccountTransactionListItemCursor,
    OwnedAccountTransactionListItemSortKey,
};

/// Query parameters for owned account transaction list reads.
#[query(name = "owned_account_transaction_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListQuery {
    pub owner: AccountOwner,
    pub criteria: OwnedAccountTransactionListItemCriteria,
    pub cursor_options: Option<
        CursorOptions<
            OwnedAccountTransactionListItemSortKey,
            OwnedAccountTransactionListItemCursor,
        >,
    >,
    pub limit: PageLimit,
}
