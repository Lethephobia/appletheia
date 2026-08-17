use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::application::unit_of_work::UnitOfWork;

use super::{
    PublicUserList, PublicUserListCriteria, PublicUserListCursor, PublicUserListReaderError,
    PublicUserListSortKey,
};

/// Loads public user list read models from query-side tables.
#[allow(async_fn_in_trait)]
pub trait PublicUserListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicUserListCriteria,
        sort: Sort<PublicUserListSortKey>,
        page: CursorPage<PublicUserListCursor>,
    ) -> Result<PublicUserList, PublicUserListReaderError>;
}
