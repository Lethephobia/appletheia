use thiserror::Error;

/// Describes why an organization website URL cannot be validated.
#[derive(Debug, Error)]
pub enum OrganizationWebsiteUrlError {
    #[error("organization website URL is invalid")]
    Parse(#[from] url::ParseError),
}
