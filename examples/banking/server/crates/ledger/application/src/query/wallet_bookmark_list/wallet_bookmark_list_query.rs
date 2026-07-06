use appletheia::query;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use crate::read_model::{
    WalletBookmarkListCriteria, WalletBookmarkListCursor, WalletBookmarkListSortKey,
};

/// Query parameters for wallet bookmark list reads.
#[query(name = "wallet_bookmark_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletBookmarkListQuery {
    pub owner: WalletBookmarkOwner,
    pub criteria: WalletBookmarkListCriteria,
    pub cursor_options: Option<CursorOptions<WalletBookmarkListSortKey, WalletBookmarkListCursor>>,
    pub limit: PageSize,
}
