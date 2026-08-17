use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::query;

use crate::read_model::{
    PublicAccountListCriteria, PublicAccountListCursor, PublicAccountListSortKey,
};

/// Query parameters for public account list reads.
#[query(name = "public_account_list")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListQuery {
    pub criteria: PublicAccountListCriteria,
    pub sort: Sort<PublicAccountListSortKey>,
    pub page: CursorPage<PublicAccountListCursor>,
}
