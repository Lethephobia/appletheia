/// Sort key for organization join request list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OrganizationJoinRequestListSortKey {
    CreatedAt,
    JoinRequestId,
}
