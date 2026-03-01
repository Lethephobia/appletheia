use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenAudiencesError {
    #[error("audiences must not be empty")]
    Empty,
}
