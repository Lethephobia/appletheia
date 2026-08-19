use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::query;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;

use crate::read_model::{
    WalletBookmarkListCriteria, WalletBookmarkListCursor, WalletBookmarkListSortKey,
};

/// Query parameters for wallet bookmark list reads.
#[query(name = "wallet_bookmark_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletBookmarkListQuery {
    pub owner: WalletBookmarkOwner,
    pub criteria: WalletBookmarkListCriteria,
    pub sort: Sort<WalletBookmarkListSortKey>,
    pub page: CursorWindow<WalletBookmarkListCursor>,
}
