use appletheia::application::unit_of_work::UnitOfWork;

use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

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
        cursor_options: Option<CursorOptions<PublicAccountListSortKey, PublicAccountListCursor>>,
        limit: PageSize,
    ) -> Result<PublicAccountList, PublicAccountListReaderError>;
}
