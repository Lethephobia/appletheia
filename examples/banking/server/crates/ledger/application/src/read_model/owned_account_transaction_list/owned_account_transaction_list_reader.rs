use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::AccountOwner;

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
        sort: Sort<OwnedAccountTransactionListSortKey>,
        page: CursorWindow<OwnedAccountTransactionListCursor>,
    ) -> Result<OwnedAccountTransactionList, OwnedAccountTransactionListReaderError>;
}
