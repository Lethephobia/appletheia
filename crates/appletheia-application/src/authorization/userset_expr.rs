use super::RelationRef;

/// Represents a userset expression in the authorization model.
///
/// A `UsersetExpr` describes how subjects for a relation are resolved from
/// direct tuples, other relations, or set operations on relations.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UsersetExpr {
    /// Read relationships for the currently-evaluated relation.
    This,

    /// Evaluate another relation on the same aggregate.
    ComputedUserset {
        /// The relation to evaluate on the current aggregate.
        relation: RelationRef,
    },

    /// `computed_relation from tupleset_relation`
    TupleToUserset {
        /// The relation that yields related aggregates to traverse.
        tupleset_relation: RelationRef,
        /// The relation to evaluate on each related aggregate.
        computed_userset: RelationRef,
    },

    /// Resolves subjects that appear in any of the contained expressions.
    Union(&'static [UsersetExpr]),

    /// Resolves only subjects that appear in all contained expressions.
    Intersection(&'static [UsersetExpr]),

    /// Resolves subjects from `base` except those also contained in `subtract`.
    Difference {
        /// The base userset expression.
        base: &'static UsersetExpr,
        /// The userset expression to subtract from the base result.
        subtract: &'static UsersetExpr,
    },
}
