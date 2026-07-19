/// Sort key for user organization join request list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserOrganizationJoinRequestListSortKey {
    CreatedAt,
    JoinRequestId,
}
