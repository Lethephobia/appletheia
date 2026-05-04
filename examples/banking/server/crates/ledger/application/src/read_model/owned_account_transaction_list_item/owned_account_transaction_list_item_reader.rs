use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::CurrencyId;

use crate::pagination::{CursorOptions, Page, PageLimit};

use super::{
    OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCursor,
    OwnedAccountTransactionListItemReaderError, OwnedAccountTransactionListItemSortKey,
    OwnedAccountTransactionListItemStatus,
};

/// Loads owned account transaction list read models.
#[allow(async_fn_in_trait, clippy::too_many_arguments)]
pub trait OwnedAccountTransactionListItemReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        account_id: Option<AccountId>,
        currency_id: Option<CurrencyId>,
        status: Option<OwnedAccountTransactionListItemStatus>,
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
