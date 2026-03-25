use serde::{Deserialize, Serialize};

/// Returned after a logout request revokes the current access token.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogoutOutput;
