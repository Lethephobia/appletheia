use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::AccountOwner;

use crate::pagination::{CursorOptions, Page, PageSize};

use super::{
    OwnedAccountListItem, OwnedAccountListItemCriteria, OwnedAccountListItemCursor,
    OwnedAccountListItemReaderError, OwnedAccountListItemSortKey,
};

/// Loads account list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait OwnedAccountListItemReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountListItemCriteria,
        cursor_options: Option<
            CursorOptions<OwnedAccountListItemSortKey, OwnedAccountListItemCursor>,
        >,
        limit: PageSize,
    ) -> Result<
        Page<OwnedAccountListItem, OwnedAccountListItemCursor>,
        OwnedAccountListItemReaderError,
    >;
}
