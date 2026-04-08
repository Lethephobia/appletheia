use serde::{Deserialize, Serialize};

/// The output returned after accepting an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationAcceptOutput;
