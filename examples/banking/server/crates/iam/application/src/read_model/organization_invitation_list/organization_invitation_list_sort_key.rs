/// Sort key for organization invitation list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OrganizationInvitationListSortKey {
    CreatedAt,
    InvitationId,
}
