use serde::{Deserialize, Serialize};

/// Explains why the server closed a subscription.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadModelWatchCloseReason {
    ClientReleased,
    SessionClosed,
    SessionExpired,
    AuthorizationDenied,
    ServerShutdown,
}
