/// Sort key for user organization invitation list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserOrganizationInvitationListSortKey {
    CreatedAt,
    InvitationId,
}
