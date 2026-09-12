use thiserror::Error;

use super::RelationshipDerivationError;

#[derive(Debug, Error)]
pub enum RelationshipDeriverError {
    #[error(transparent)]
    RelationshipDerivation(#[from] RelationshipDerivationError),
}
