use serde::{Deserialize, Serialize};

/// Returned after a user bio change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBioChangeOutput;
