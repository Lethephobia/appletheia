use serde::{Deserialize, Serialize};

/// Returned after a user picture object delete request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPictureObjectDeleteOutput;
