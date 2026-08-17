use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::application::unit_of_work::UnitOfWork;

use super::{
    PublicAccountList, PublicAccountListCriteria, PublicAccountListCursor,
    PublicAccountListReaderError, PublicAccountListSortKey,
};

/// Loads public account list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait PublicAccountListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicAccountListCriteria,
        sort: Sort<PublicAccountListSortKey>,
        page: CursorPage<PublicAccountListCursor>,
    ) -> Result<PublicAccountList, PublicAccountListReaderError>;
}
