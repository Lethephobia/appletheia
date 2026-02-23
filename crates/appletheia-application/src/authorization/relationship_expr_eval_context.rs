use super::{AggregateRef, RelationName};

pub(super) struct RelationshipExprEvalContext<'a> {
    pub(super) subject: &'a AggregateRef,
    pub(super) aggregate: &'a AggregateRef,
    pub(super) current_relation: &'a RelationName,
    pub(super) depth: usize,
}

impl<'a> RelationshipExprEvalContext<'a> {
    pub(super) fn new(
        subject: &'a AggregateRef,
        aggregate: &'a AggregateRef,
        current_relation: &'a RelationName,
        depth: usize,
    ) -> Self {
        Self {
            subject,
            aggregate,
            current_relation,
            depth,
        }
    }
}
