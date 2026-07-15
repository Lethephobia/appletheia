use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

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
        cursor_options: Option<CursorOptions<WalletBookmarkListSortKey, WalletBookmarkListCursor>>,
        limit: PageSize,
    ) -> Result<WalletBookmarkList, WalletBookmarkListReaderError>;
}
