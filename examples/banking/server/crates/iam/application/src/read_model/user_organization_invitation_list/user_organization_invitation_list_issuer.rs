use banking_iam_domain::UserId;

/// Issuer shown in a user organization invitation list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserOrganizationInvitationListIssuer {
    User(UserId),
    System,
}
