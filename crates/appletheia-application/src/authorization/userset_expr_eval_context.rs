use super::{AggregateRef, RelationName};

use super::userset_expr_eval_depth::UsersetExprEvalDepth;

pub struct UsersetExprEvalContext<'a> {
    pub subject: &'a AggregateRef,
    pub aggregate: &'a AggregateRef,
    pub relation: &'a RelationName,
    pub depth: UsersetExprEvalDepth,
}

impl<'a> UsersetExprEvalContext<'a> {
    pub fn new(
        subject: &'a AggregateRef,
        aggregate: &'a AggregateRef,
        relation: &'a RelationName,
        depth: UsersetExprEvalDepth,
    ) -> Self {
        Self {
            subject,
            aggregate,
            relation,
            depth,
        }
    }
}
