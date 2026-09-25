use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::UserId;

use super::{
    UserOrganizationMembershipList, UserOrganizationMembershipListCursor,
    UserOrganizationMembershipListReaderError, UserOrganizationMembershipListSortKey,
};

/// Loads user organization membership lists.
#[allow(async_fn_in_trait)]
pub trait UserOrganizationMembershipListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        sort: Sort<UserOrganizationMembershipListSortKey>,
        page: CursorWindow<UserOrganizationMembershipListCursor>,
    ) -> Result<UserOrganizationMembershipList, UserOrganizationMembershipListReaderError>;
}
