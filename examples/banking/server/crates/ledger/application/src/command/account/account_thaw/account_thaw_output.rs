use serde::{Deserialize, Serialize};

/// Returned after an account thaw request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountThawOutput;
