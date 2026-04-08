use serde::{Deserialize, Serialize};

/// The output returned after declining an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationDeclineOutput;
