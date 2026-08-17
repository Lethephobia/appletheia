use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::UserId;

use super::{
    UserOrganizationJoinRequestList, UserOrganizationJoinRequestListCriteria,
    UserOrganizationJoinRequestListCursor, UserOrganizationJoinRequestListReaderError,
    UserOrganizationJoinRequestListSortKey,
};

/// Loads user organization join request lists.
#[allow(async_fn_in_trait)]
pub trait UserOrganizationJoinRequestListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        criteria: UserOrganizationJoinRequestListCriteria,
        sort: Sort<UserOrganizationJoinRequestListSortKey>,
        page: CursorPage<UserOrganizationJoinRequestListCursor>,
    ) -> Result<UserOrganizationJoinRequestList, UserOrganizationJoinRequestListReaderError>;
}
