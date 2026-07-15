use appletheia::command;
use serde::{Deserialize, Serialize};

/// Configures the on-chain ledger backend.
#[command(name = "onchain_configure")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnchainConfigureCommand;
