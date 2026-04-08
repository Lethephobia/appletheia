use crate::event::AggregateTypeOwned;

use std::error::Error;

use thiserror::Error as ThisError;

use super::{RelationRefOwned, RelationshipStoreError};

#[derive(Debug, ThisError)]
pub enum RelationshipResolverError {
    #[error("relationship store error: {0}")]
    RelationshipStore(#[from] RelationshipStoreError),

    #[error("relationship resolver evaluation limit exceeded: {0}")]
    EvaluationLimitExceeded(&'static str),

    #[error(
        "relationship reference aggregate type does not match target aggregate: target={aggregate_type}, relation={relation}"
    )]
    InvalidRelationReference {
        aggregate_type: AggregateTypeOwned,
        relation: RelationRefOwned,
    },

    #[error("relationship resolver backend error: {0}")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}

impl RelationshipResolverError {
    pub fn backend<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::Backend(Box::new(error))
    }
}
