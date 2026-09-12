use super::RelationRefOwned;
use crate::aggregate::AggregateRef;

use super::userset_expr_eval_depth::UsersetExprEvalDepth;

pub struct UsersetExprEvalContext<'a> {
    pub subject: &'a AggregateRef,
    pub target: &'a AggregateRef,
    pub relation: &'a RelationRefOwned,
    pub depth: UsersetExprEvalDepth,
}

impl<'a> UsersetExprEvalContext<'a> {
    pub fn new(
        subject: &'a AggregateRef,
        target: &'a AggregateRef,
        relation: &'a RelationRefOwned,
        depth: UsersetExprEvalDepth,
    ) -> Self {
        Self {
            subject,
            target,
            relation,
            depth,
        }
    }
}
