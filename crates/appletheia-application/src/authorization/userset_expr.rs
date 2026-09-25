use crate::aggregate::SerializedAggregateError;
use appletheia_domain::Aggregate;

use super::{RelationRefOwned, RelationshipDerivationSource, RelationshipEntries};

/// Owns a userset expression for runtime authorization evaluation.
#[derive(Clone, Debug)]
pub enum UsersetExpr {
    /// Reads direct persisted tuples during evaluation. Sources run only during save.
    /// An empty list declares a read-only direct relation with no local derivation.
    This(Vec<RelationshipDerivationSource>),
    ComputedUserset {
        relation: RelationRefOwned,
    },
    TupleToUserset {
        tupleset_relation: RelationRefOwned,
        computed_userset: RelationRefOwned,
    },
    Union(Vec<UsersetExpr>),
    Intersection(Vec<UsersetExpr>),
    Difference {
        base: Box<UsersetExpr>,
        subtract: Box<UsersetExpr>,
    },
}

impl UsersetExpr {
    /// Declares direct tuples derived from the current state of source aggregate `A`.
    pub fn this<A, F, E>(handler: F) -> Self
    where
        A: Aggregate + 'static,
        F: Fn(&A) -> Result<RelationshipEntries, E> + Send + Sync + 'static,
        E: std::error::Error + From<SerializedAggregateError> + Send + Sync + 'static,
    {
        Self::This(vec![RelationshipDerivationSource::new::<A, F, E>(handler)])
    }
}
