use std::error::Error;
use std::fmt;
use std::sync::Arc;

use appletheia_domain::Aggregate;

use crate::aggregate::{AggregateTypeOwned, SerializedAggregateError};

use super::RelationshipEntries;
use super::relationship_derivation_handler::RelationshipDerivationHandler;

/// A typed aggregate callback adapted to a shared serialized input.
#[derive(Clone)]
pub struct RelationshipDerivationSource {
    pub(crate) aggregate_type: AggregateTypeOwned,
    pub(crate) handler: RelationshipDerivationHandler,
}

impl RelationshipDerivationSource {
    pub fn new<A, F, E>(handler: F) -> Self
    where
        A: Aggregate + 'static,
        F: Fn(&A) -> Result<RelationshipEntries, E> + Send + Sync + 'static,
        E: Error + From<SerializedAggregateError> + Send + Sync + 'static,
    {
        let shared_handler: RelationshipDerivationHandler = Arc::new(move |serialized| {
            let aggregate = serialized.try_into_aggregate::<A>().map_err(E::from)?;
            handler(&aggregate).map_err(Into::into)
        });
        Self {
            aggregate_type: AggregateTypeOwned::from(A::TYPE),
            handler: shared_handler,
        }
    }
}

impl fmt::Debug for RelationshipDerivationSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RelationshipDerivationSource")
            .field("aggregate_type", &self.aggregate_type)
            .finish_non_exhaustive()
    }
}
