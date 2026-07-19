use appletheia::application::unit_of_work::UnitOfWork;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

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
        cursor_options: Option<CursorOptions<PublicUserListSortKey, PublicUserListCursor>>,
        limit: PageSize,
    ) -> Result<PublicUserList, PublicUserListReaderError>;
}
