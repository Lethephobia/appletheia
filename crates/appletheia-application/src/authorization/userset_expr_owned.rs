use super::{RelationRefOwned, UsersetExpr};

/// Owns a userset expression for runtime authorization evaluation.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UsersetExprOwned {
    This,
    ComputedUserset {
        relation: RelationRefOwned,
    },
    TupleToUserset {
        tupleset_relation: RelationRefOwned,
        computed_userset: RelationRefOwned,
    },
    Union(Vec<UsersetExprOwned>),
    Intersection(Vec<UsersetExprOwned>),
    Difference {
        base: Box<UsersetExprOwned>,
        subtract: Box<UsersetExprOwned>,
    },
}

impl From<&UsersetExpr> for UsersetExprOwned {
    fn from(value: &UsersetExpr) -> Self {
        match value {
            UsersetExpr::This => Self::This,
            UsersetExpr::ComputedUserset { relation } => Self::ComputedUserset {
                relation: (*relation).into(),
            },
            UsersetExpr::TupleToUserset {
                tupleset_relation,
                computed_userset,
            } => Self::TupleToUserset {
                tupleset_relation: (*tupleset_relation).into(),
                computed_userset: (*computed_userset).into(),
            },
            UsersetExpr::Union(items) => {
                Self::Union(items.iter().map(UsersetExprOwned::from).collect())
            }
            UsersetExpr::Intersection(items) => {
                Self::Intersection(items.iter().map(UsersetExprOwned::from).collect())
            }
            UsersetExpr::Difference { base, subtract } => Self::Difference {
                base: Box::new(UsersetExprOwned::from(*base)),
                subtract: Box::new(UsersetExprOwned::from(*subtract)),
            },
        }
    }
}
