use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::AccountOwner;

use super::{
    OwnedAccountList, OwnedAccountListCriteria, OwnedAccountListCursor,
    OwnedAccountListReaderError, OwnedAccountListSortKey,
};

/// Loads account list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait OwnedAccountListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountListCriteria,
        sort: Sort<OwnedAccountListSortKey>,
        page: CursorPage<OwnedAccountListCursor>,
    ) -> Result<OwnedAccountList, OwnedAccountListReaderError>;
}
