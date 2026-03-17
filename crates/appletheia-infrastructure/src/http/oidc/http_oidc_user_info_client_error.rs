use thiserror::Error;

/// Describes HTTP user-info client failures.
#[derive(Debug, Error)]
pub enum HttpOidcUserInfoClientError {
    #[error("user info endpoint returned non-success status: {status}")]
    UnexpectedStatus { status: u16, body: String },
}
