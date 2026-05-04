use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::AccountOwner;
use banking_ledger_domain::currency::CurrencyId;

use crate::pagination::{CursorOptions, Page, PageLimit};

use super::{
    OwnedAccountListItem, OwnedAccountListItemCursor, OwnedAccountListItemReaderError,
    OwnedAccountListItemSortKey, OwnedAccountListItemStatus,
};

/// Loads account list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait OwnedAccountListItemReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        currency_id: Option<CurrencyId>,
        status: Option<OwnedAccountListItemStatus>,
        cursor_options: Option<
            CursorOptions<OwnedAccountListItemSortKey, OwnedAccountListItemCursor>,
        >,
        limit: PageLimit,
    ) -> Result<
        Page<OwnedAccountListItem, OwnedAccountListItemCursor>,
        OwnedAccountListItemReaderError,
    >;
}
