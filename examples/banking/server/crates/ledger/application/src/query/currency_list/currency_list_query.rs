use appletheia::query;

use crate::pagination::{CursorOptions, PageSize};
use crate::read_model::{
    CurrencyListItemCriteria, CurrencyListItemCursor, CurrencyListItemSortKey,
};

/// Query parameters for public currency list reads.
#[query(name = "currency_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListQuery {
    pub criteria: CurrencyListItemCriteria,
    pub cursor_options: Option<CursorOptions<CurrencyListItemSortKey, CurrencyListItemCursor>>,
    pub limit: PageSize,
}
