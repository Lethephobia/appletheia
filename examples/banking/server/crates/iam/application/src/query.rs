mod organization_internal_info;
mod organization_invitation_list;
mod organization_join_request_list;
mod organization_management_info;
mod organization_member_list;
mod public_organization_list;
mod public_user_list;
mod user_organization_invitation_list;
mod user_organization_join_request_list;
mod user_organization_membership_list;
mod user_private_info;
mod user_public_profile;

pub use organization_internal_info::{
    OrganizationInternalInfoQuery, OrganizationInternalInfoQueryHandler,
    OrganizationInternalInfoQueryHandlerError,
};
pub use organization_invitation_list::{
    OrganizationInvitationListQuery, OrganizationInvitationListQueryHandler,
    OrganizationInvitationListQueryHandlerError,
};
pub use organization_join_request_list::{
    OrganizationJoinRequestListQuery, OrganizationJoinRequestListQueryHandler,
    OrganizationJoinRequestListQueryHandlerError,
};
pub use organization_management_info::{
    OrganizationManagementInfoQuery, OrganizationManagementInfoQueryHandler,
    OrganizationManagementInfoQueryHandlerError,
};
pub use organization_member_list::{
    OrganizationMemberListQuery, OrganizationMemberListQueryHandler,
    OrganizationMemberListQueryHandlerError,
};
pub use public_organization_list::{
    PublicOrganizationListQuery, PublicOrganizationListQueryHandler,
    PublicOrganizationListQueryHandlerError,
};
pub use public_user_list::{
    PublicUserListQuery, PublicUserListQueryHandler, PublicUserListQueryHandlerError,
};
pub use user_organization_invitation_list::{
    UserOrganizationInvitationListQuery, UserOrganizationInvitationListQueryHandler,
    UserOrganizationInvitationListQueryHandlerError,
};
pub use user_organization_join_request_list::{
    UserOrganizationJoinRequestListQuery, UserOrganizationJoinRequestListQueryHandler,
    UserOrganizationJoinRequestListQueryHandlerError,
};
pub use user_organization_membership_list::{
    UserOrganizationMembershipListQuery, UserOrganizationMembershipListQueryHandler,
    UserOrganizationMembershipListQueryHandlerError,
};
pub use user_private_info::{
    UserPrivateInfoQuery, UserPrivateInfoQueryHandler, UserPrivateInfoQueryHandlerError,
};
pub use user_public_profile::{
    UserPublicProfileQuery, UserPublicProfileQueryHandler, UserPublicProfileQueryHandlerError,
};

use appletheia::application::query::{QueryHandler, WatchableQueryHandler};
use appletheia::application::read_model::{ReadModelDependency, ReadModelDependencyTopic};

use crate::projection::{
    OrganizationFragment, OrganizationInvitationFragment, OrganizationJoinRequestFragment,
    OrganizationMembershipFragment, UserFragment, UserIdentityFragment,
};
use crate::read_model::{
    OrganizationInternalInfoReader, OrganizationInvitationListReader,
    OrganizationJoinRequestListReader, OrganizationManagementInfoReader,
    OrganizationMemberListReader, PublicOrganizationListReader, PublicUserListReader,
    UserOrganizationInvitationListReader, UserOrganizationJoinRequestListReader,
    UserPrivateInfoReader, UserPublicProfileReader,
};

macro_rules! impl_watchable_query_handler {
    ($handler:ident, $reader:path, [$($fragment:ty),+ $(,)?]) => {
        impl<T> WatchableQueryHandler for $handler<T>
        where
            T: $reader,
            $handler<T>: QueryHandler,
        {
            fn watch_dependencies(
                &self,
                _query: &Self::Query,
            ) -> Result<Vec<ReadModelDependency>, Self::Error> {
                Ok(vec![$(
                    ReadModelDependency::Topic(ReadModelDependencyTopic::all::<$fragment>()),
                )+])
            }
        }
    };
}

impl_watchable_query_handler!(
    OrganizationInternalInfoQueryHandler,
    OrganizationInternalInfoReader,
    [OrganizationFragment]
);
impl_watchable_query_handler!(
    OrganizationInvitationListQueryHandler,
    OrganizationInvitationListReader,
    [
        OrganizationFragment,
        OrganizationInvitationFragment,
        UserFragment,
    ]
);
impl_watchable_query_handler!(
    OrganizationJoinRequestListQueryHandler,
    OrganizationJoinRequestListReader,
    [
        OrganizationFragment,
        OrganizationJoinRequestFragment,
        UserFragment,
    ]
);
impl_watchable_query_handler!(
    OrganizationManagementInfoQueryHandler,
    OrganizationManagementInfoReader,
    [OrganizationFragment, UserFragment]
);
impl_watchable_query_handler!(
    OrganizationMemberListQueryHandler,
    OrganizationMemberListReader,
    [
        OrganizationFragment,
        OrganizationMembershipFragment,
        UserFragment,
    ]
);
impl_watchable_query_handler!(
    PublicOrganizationListQueryHandler,
    PublicOrganizationListReader,
    [OrganizationFragment]
);
impl_watchable_query_handler!(
    PublicUserListQueryHandler,
    PublicUserListReader,
    [UserFragment]
);
impl_watchable_query_handler!(
    UserOrganizationInvitationListQueryHandler,
    UserOrganizationInvitationListReader,
    [
        UserFragment,
        OrganizationInvitationFragment,
        OrganizationFragment,
    ]
);
impl_watchable_query_handler!(
    UserOrganizationJoinRequestListQueryHandler,
    UserOrganizationJoinRequestListReader,
    [
        UserFragment,
        OrganizationJoinRequestFragment,
        OrganizationFragment,
    ]
);
impl_watchable_query_handler!(
    UserPrivateInfoQueryHandler,
    UserPrivateInfoReader,
    [
        UserFragment,
        UserIdentityFragment,
        OrganizationMembershipFragment,
        OrganizationFragment,
    ]
);
impl_watchable_query_handler!(
    UserPublicProfileQueryHandler,
    UserPublicProfileReader,
    [UserFragment]
);
