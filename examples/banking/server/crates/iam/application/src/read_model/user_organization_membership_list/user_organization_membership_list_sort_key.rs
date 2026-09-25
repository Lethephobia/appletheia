/// Sort key for user organization membership list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserOrganizationMembershipListSortKey {
    CreatedAt,
    OrganizationMembershipId,
}
