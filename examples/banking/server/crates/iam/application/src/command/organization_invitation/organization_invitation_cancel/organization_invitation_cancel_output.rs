use serde::{Deserialize, Serialize};

/// The output returned after canceling an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationCancelOutput;
