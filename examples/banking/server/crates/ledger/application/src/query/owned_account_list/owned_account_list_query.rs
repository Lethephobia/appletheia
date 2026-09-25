use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::query;
use banking_ledger_domain::account::AccountOwner;

use crate::read_model::{
    OwnedAccountListCriteria, OwnedAccountListCursor, OwnedAccountListSortKey,
};

/// Query parameters for account list reads.
#[query(name = "owned_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListQuery {
    pub owner: AccountOwner,
    pub criteria: OwnedAccountListCriteria,
    pub sort: Sort<OwnedAccountListSortKey>,
    pub page: CursorWindow<OwnedAccountListCursor>,
}
