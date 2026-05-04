use appletheia::application::unit_of_work::UnitOfWork;

use crate::pagination::{CursorOptions, Page, PageLimit};

use super::{
    CurrencyListItem, CurrencyListItemCursor, CurrencyListItemReaderError, CurrencyListItemSortKey,
    CurrencyListItemStatus,
};

/// Loads currency list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait CurrencyListItemReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        status: Option<CurrencyListItemStatus>,
        cursor_options: Option<CursorOptions<CurrencyListItemSortKey, CurrencyListItemCursor>>,
        limit: PageLimit,
    ) -> Result<Page<CurrencyListItem, CurrencyListItemCursor>, CurrencyListItemReaderError>;
}
