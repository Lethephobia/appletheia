use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpOidcTokenClientError {
    #[error("token endpoint returned non-success status: {status}")]
    UnexpectedStatus { status: u16, body: String },
}
