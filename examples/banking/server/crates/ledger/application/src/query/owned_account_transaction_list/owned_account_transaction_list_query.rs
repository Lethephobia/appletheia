use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::query;
use banking_ledger_domain::account::AccountOwner;

use crate::read_model::{
    OwnedAccountTransactionListCriteria, OwnedAccountTransactionListCursor,
    OwnedAccountTransactionListSortKey,
};

/// Query parameters for owned account transaction list reads.
#[query(name = "owned_account_transaction_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListQuery {
    pub owner: AccountOwner,
    pub criteria: OwnedAccountTransactionListCriteria,
    pub sort: Sort<OwnedAccountTransactionListSortKey>,
    pub page: CursorWindow<OwnedAccountTransactionListCursor>,
}
