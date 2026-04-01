use serde::{Deserialize, Serialize};

/// The output returned after renaming an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountRenameOutput;
