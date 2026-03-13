use super::{AggregateRef, RelationNameOwned};

use super::userset_expr_eval_depth::UsersetExprEvalDepth;

pub struct UsersetExprEvalContext<'a> {
    pub subject: &'a AggregateRef,
    pub aggregate: &'a AggregateRef,
    pub relation: &'a RelationNameOwned,
    pub depth: UsersetExprEvalDepth,
}

impl<'a> UsersetExprEvalContext<'a> {
    pub fn new(
        subject: &'a AggregateRef,
        aggregate: &'a AggregateRef,
        relation: &'a RelationNameOwned,
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
