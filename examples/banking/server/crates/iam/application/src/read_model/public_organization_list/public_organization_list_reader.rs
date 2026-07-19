use appletheia::application::unit_of_work::UnitOfWork;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use super::{
    PublicOrganizationList, PublicOrganizationListCriteria, PublicOrganizationListCursor,
    PublicOrganizationListReaderError, PublicOrganizationListSortKey,
};

/// Loads public organization list read models from query-side tables.
#[allow(async_fn_in_trait)]
pub trait PublicOrganizationListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicOrganizationListCriteria,
        cursor_options: Option<
            CursorOptions<PublicOrganizationListSortKey, PublicOrganizationListCursor>,
        >,
        limit: PageSize,
    ) -> Result<PublicOrganizationList, PublicOrganizationListReaderError>;
}
