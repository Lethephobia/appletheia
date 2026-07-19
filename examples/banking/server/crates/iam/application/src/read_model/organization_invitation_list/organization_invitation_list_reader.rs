use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::OrganizationId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use super::{
    OrganizationInvitationList, OrganizationInvitationListCriteria,
    OrganizationInvitationListCursor, OrganizationInvitationListReaderError,
    OrganizationInvitationListSortKey,
};

/// Loads organization invitation lists.
#[allow(async_fn_in_trait)]
pub trait OrganizationInvitationListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        criteria: OrganizationInvitationListCriteria,
        cursor_options: Option<
            CursorOptions<OrganizationInvitationListSortKey, OrganizationInvitationListCursor>,
        >,
        limit: PageSize,
    ) -> Result<OrganizationInvitationList, OrganizationInvitationListReaderError>;
}
