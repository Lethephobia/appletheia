use serde::{Deserialize, Serialize};

/// Returned after a username change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUsernameChangeOutput;
