use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::AccountOwner;

use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use super::{
    OwnedAccountTransactionList, OwnedAccountTransactionListCriteria,
    OwnedAccountTransactionListCursor, OwnedAccountTransactionListReaderError,
    OwnedAccountTransactionListSortKey,
};

/// Loads owned account transaction list read models.
#[allow(async_fn_in_trait)]
pub trait OwnedAccountTransactionListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountTransactionListCriteria,
        cursor_options: Option<
            CursorOptions<OwnedAccountTransactionListSortKey, OwnedAccountTransactionListCursor>,
        >,
        limit: PageSize,
    ) -> Result<OwnedAccountTransactionList, OwnedAccountTransactionListReaderError>;
}
