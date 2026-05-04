use appletheia::query;

use crate::pagination::{CursorOptions, PageLimit};
use crate::read_model::{CurrencyListItemCursor, CurrencyListItemSortKey, CurrencyListItemStatus};

/// Query parameters for public currency list reads.
#[query(name = "currency_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListQuery {
    pub status: Option<CurrencyListItemStatus>,
    pub cursor_options: Option<CursorOptions<CurrencyListItemSortKey, CurrencyListItemCursor>>,
    pub limit: PageLimit,
}
