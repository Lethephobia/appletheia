use serde::{Deserialize, Serialize};

/// Represents why an OIDC completion flow is being performed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OidcCompletionPurpose {
    Token,
    ExchangeCode,
    LinkIdentity,
}
