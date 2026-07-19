/// Lifecycle status shown in a user organization invitation list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserOrganizationInvitationListItemStatus {
    Pending,
    Accepted,
    Declined,
    Canceled,
    Rejected,
}
