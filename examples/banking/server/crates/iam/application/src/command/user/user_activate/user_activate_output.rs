use serde::{Deserialize, Serialize};

/// Returned after a user activation request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserActivateOutput;
