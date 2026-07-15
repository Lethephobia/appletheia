use appletheia::application::unit_of_work::UnitOfWork;

use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use super::{
    CurrencyList, CurrencyListCriteria, CurrencyListCursor, CurrencyListReaderError,
    CurrencyListSortKey,
};

/// Loads currency list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait CurrencyListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: CurrencyListCriteria,
        cursor_options: Option<CursorOptions<CurrencyListSortKey, CurrencyListCursor>>,
        limit: PageSize,
    ) -> Result<CurrencyList, CurrencyListReaderError>;
}
