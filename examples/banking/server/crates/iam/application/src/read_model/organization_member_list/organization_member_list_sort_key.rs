/// Sort key for organization member list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OrganizationMemberListSortKey {
    JoinedAt,
    UserId,
}
