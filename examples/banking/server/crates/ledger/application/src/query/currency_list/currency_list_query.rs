use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::query;

use crate::read_model::{CurrencyListCriteria, CurrencyListCursor, CurrencyListSortKey};

/// Query parameters for public currency list reads.
#[query(name = "currency_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListQuery {
    pub criteria: CurrencyListCriteria,
    pub sort: Sort<CurrencyListSortKey>,
    pub page: CursorPage<CurrencyListCursor>,
}
