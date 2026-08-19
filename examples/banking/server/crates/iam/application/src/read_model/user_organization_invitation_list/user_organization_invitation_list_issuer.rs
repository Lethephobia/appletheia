use serde::Serialize;

use banking_iam_domain::UserId;

/// Issuer shown in a user organization invitation list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum UserOrganizationInvitationListIssuer {
    User(UserId),
    System,
}
