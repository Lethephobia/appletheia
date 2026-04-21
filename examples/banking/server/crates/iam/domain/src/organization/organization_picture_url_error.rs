use thiserror::Error;

/// Describes why an organization picture URL cannot be validated.
#[derive(Debug, Error)]
pub enum OrganizationPictureUrlError {
    #[error("organization picture URL is invalid")]
    Parse(#[from] url::ParseError),
}
