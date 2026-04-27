use serde::{Deserialize, Serialize};

/// Returned after a user picture change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureChangeOutput;
