use super::RelationNameOwned;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UsersetExpr {
    /// Read relationships for the currently-evaluated relation.
    This,

    /// Evaluate another relation on the same aggregate.
    ComputedUserset {
        relation: RelationNameOwned,
    },

    /// `computed_relation from tupleset_relation`
    TupleToUserset {
        tupleset_relation: RelationNameOwned,
        computed_relation: RelationNameOwned,
    },

    Union(Vec<UsersetExpr>),

    Intersection(Vec<UsersetExpr>),

    Difference {
        base: Box<UsersetExpr>,
        subtract: Box<UsersetExpr>,
    },
}
