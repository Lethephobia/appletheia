use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::application::unit_of_work::UnitOfWork;

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
        sort: Sort<CurrencyListSortKey>,
        page: CursorPage<CurrencyListCursor>,
    ) -> Result<CurrencyList, CurrencyListReaderError>;
}
