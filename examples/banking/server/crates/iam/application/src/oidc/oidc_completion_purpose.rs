use serde::{Deserialize, Serialize};

/// Represents why an OIDC completion flow is being performed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OidcCompletionPurpose {
    Token,
    ExchangeCode,
    LinkIdentity,
}
