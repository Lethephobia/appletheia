use serde::{Deserialize, Serialize};

/// The output returned after rejecting an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestRejectOutput;
