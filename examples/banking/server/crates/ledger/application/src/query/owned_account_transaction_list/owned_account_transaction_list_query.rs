use appletheia::query;
use banking_ledger_domain::account::AccountOwner;

use crate::read_model::{
    OwnedAccountTransactionListCriteria, OwnedAccountTransactionListCursor,
    OwnedAccountTransactionListSortKey,
};
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

/// Query parameters for owned account transaction list reads.
#[query(name = "owned_account_transaction_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListQuery {
    pub owner: AccountOwner,
    pub criteria: OwnedAccountTransactionListCriteria,
    pub cursor_options: Option<
        CursorOptions<OwnedAccountTransactionListSortKey, OwnedAccountTransactionListCursor>,
    >,
    pub limit: PageSize,
}
