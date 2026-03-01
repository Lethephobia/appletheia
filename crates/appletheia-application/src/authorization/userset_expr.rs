use super::RelationName;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UsersetExpr {
    /// Read relationships for the currently-evaluated relation.
    This,

    /// Evaluate another relation on the same aggregate.
    ComputedUserset {
        relation: RelationName,
    },

    /// `computed_relation from tupleset_relation`
    TupleToUserset {
        tupleset_relation: RelationName,
        computed_relation: RelationName,
    },

    Union(Vec<UsersetExpr>),

    Intersection(Vec<UsersetExpr>),

    Difference {
        base: Box<UsersetExpr>,
        subtract: Box<UsersetExpr>,
    },
}
