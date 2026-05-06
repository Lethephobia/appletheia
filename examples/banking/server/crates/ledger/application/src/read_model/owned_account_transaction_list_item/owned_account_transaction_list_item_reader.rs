use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::AccountOwner;

use crate::pagination::{CursorOptions, Page, PageLimit};

use super::{
    OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCriteria,
    OwnedAccountTransactionListItemCursor, OwnedAccountTransactionListItemReaderError,
    OwnedAccountTransactionListItemSortKey,
};

/// Loads owned account transaction list read models.
#[allow(async_fn_in_trait, clippy::too_many_arguments)]
pub trait OwnedAccountTransactionListItemReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountTransactionListItemCriteria,
        cursor_options: Option<
            CursorOptions<
                OwnedAccountTransactionListItemSortKey,
                OwnedAccountTransactionListItemCursor,
            >,
        >,
        limit: PageLimit,
    ) -> Result<
        Page<OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCursor>,
        OwnedAccountTransactionListItemReaderError,
    >;
}
