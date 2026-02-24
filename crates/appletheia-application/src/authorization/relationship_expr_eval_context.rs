use super::{AggregateRef, RelationName};

pub struct RelationshipExprEvalContext<'a> {
    pub subject: &'a AggregateRef,
    pub aggregate: &'a AggregateRef,
    pub current_relation: &'a RelationName,
    pub depth: usize,
}

impl<'a> RelationshipExprEvalContext<'a> {
    pub fn new(
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
