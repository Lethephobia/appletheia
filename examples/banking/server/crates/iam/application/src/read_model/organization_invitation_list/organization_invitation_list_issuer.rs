use serde::Serialize;

use banking_iam_domain::UserId;

/// Issuer shown in an organization invitation list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum OrganizationInvitationListIssuer {
    User(UserId),
    System,
}
