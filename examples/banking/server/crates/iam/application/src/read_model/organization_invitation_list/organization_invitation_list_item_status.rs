/// Lifecycle status shown in an organization invitation list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OrganizationInvitationListItemStatus {
    Pending,
    Accepted,
    Declined,
    Canceled,
    Rejected,
}
