use serde::{Deserialize, Serialize};

/// The output returned after readying a user's profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileReadyOutput;
