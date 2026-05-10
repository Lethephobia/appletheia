use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::AccountOwner;

use crate::read_model::{CursorOptions, PageSize};

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
        cursor_options: Option<CursorOptions<OwnedAccountListSortKey, OwnedAccountListCursor>>,
        limit: PageSize,
    ) -> Result<OwnedAccountList, OwnedAccountListReaderError>;
}
