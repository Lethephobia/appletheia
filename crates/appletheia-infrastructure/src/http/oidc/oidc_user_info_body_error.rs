use thiserror::Error;

/// Describes user-info JSON decoding failures.
#[derive(Debug, Error)]
pub enum OidcUserInfoBodyError {
    #[error("invalid user info json")]
    InvalidJson(#[source] serde_json::Error),

    #[error("invalid user info field: {field}")]
    InvalidField {
        field: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
