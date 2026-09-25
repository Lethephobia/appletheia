use serde::{Deserialize, Serialize};

use banking_iam_domain::UserId;

/// Identifies who issued an currency registrar invitation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarInvitationIssuer {
    User(UserId),
    System,
}
