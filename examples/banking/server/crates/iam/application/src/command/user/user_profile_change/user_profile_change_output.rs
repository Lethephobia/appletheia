use serde::{Deserialize, Serialize};

/// Returned after a user profile change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileChangeOutput;
