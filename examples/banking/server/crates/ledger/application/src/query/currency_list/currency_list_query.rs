use appletheia::query;

use crate::read_model::{CurrencyListCriteria, CurrencyListCursor, CurrencyListSortKey};
use crate::read_model::{CursorOptions, PageSize};

/// Query parameters for public currency list reads.
#[query(name = "currency_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListQuery {
    pub criteria: CurrencyListCriteria,
    pub cursor_options: Option<CursorOptions<CurrencyListSortKey, CurrencyListCursor>>,
    pub limit: PageSize,
}
