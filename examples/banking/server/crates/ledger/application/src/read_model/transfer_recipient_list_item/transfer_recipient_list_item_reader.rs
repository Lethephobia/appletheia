use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::currency::CurrencyId;

use crate::pagination::{CursorOptions, Page, PageLimit};

use super::{
    TransferRecipientListItem, TransferRecipientListItemCursor,
    TransferRecipientListItemReaderError, TransferRecipientListItemSortKey,
};

/// Loads transfer recipient list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait TransferRecipientListItemReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        keyword: Option<String>,
        currency_id: Option<CurrencyId>,
        cursor_options: Option<
            CursorOptions<TransferRecipientListItemSortKey, TransferRecipientListItemCursor>,
        >,
        limit: PageLimit,
    ) -> Result<
        Page<TransferRecipientListItem, TransferRecipientListItemCursor>,
        TransferRecipientListItemReaderError,
    >;
}
