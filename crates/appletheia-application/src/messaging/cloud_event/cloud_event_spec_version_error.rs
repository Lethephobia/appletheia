use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventSpecVersionError {
    #[error("unsupported CloudEvents spec version")]
    UnsupportedVersion,
}
