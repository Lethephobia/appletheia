use serde::{Deserialize, Serialize};

/// Represents how OIDC completion should return credentials.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OidcCompletionMode {
    Token,
    ExchangeCode,
}
