use serde::{Deserialize, Serialize};

/// Explains why a command failure became terminal without exposing handler diagnostics.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandTerminalReason {
    NonRetryable,
    RetryExhausted,
}
