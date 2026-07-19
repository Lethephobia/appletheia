use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::UserId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

use super::{
    UserOrganizationInvitationList, UserOrganizationInvitationListCriteria,
    UserOrganizationInvitationListCursor, UserOrganizationInvitationListReaderError,
    UserOrganizationInvitationListSortKey,
};

/// Loads user organization invitation lists.
#[allow(async_fn_in_trait)]
pub trait UserOrganizationInvitationListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        criteria: UserOrganizationInvitationListCriteria,
        cursor_options: Option<
            CursorOptions<
                UserOrganizationInvitationListSortKey,
                UserOrganizationInvitationListCursor,
            >,
        >,
        page_size: PageSize,
    ) -> Result<UserOrganizationInvitationList, UserOrganizationInvitationListReaderError>;
}
