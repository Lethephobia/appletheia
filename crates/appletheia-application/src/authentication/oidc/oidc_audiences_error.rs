use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum OidcAudiencesError {
    #[error("audiences must not be empty")]
    Empty,
}
