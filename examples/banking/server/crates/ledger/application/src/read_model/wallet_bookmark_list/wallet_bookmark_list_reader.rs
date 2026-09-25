use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;

use super::{
    WalletBookmarkList, WalletBookmarkListCriteria, WalletBookmarkListCursor,
    WalletBookmarkListReaderError, WalletBookmarkListSortKey,
};

/// Loads wallet bookmark list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait WalletBookmarkListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: WalletBookmarkOwner,
        criteria: WalletBookmarkListCriteria,
        sort: Sort<WalletBookmarkListSortKey>,
        page: CursorWindow<WalletBookmarkListCursor>,
    ) -> Result<WalletBookmarkList, WalletBookmarkListReaderError>;
}
