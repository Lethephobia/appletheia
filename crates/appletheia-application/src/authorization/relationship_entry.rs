use appletheia_domain::Aggregate;

use super::RelationshipSubject;
use crate::aggregate::AggregateRef;

/// A direct relationship target and subject; its relation is supplied by the declaration.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelationshipEntry {
    pub target: AggregateRef,
    pub subject: RelationshipSubject,
}

impl RelationshipEntry {
    pub fn new<Target, Subject>(target_id: Target::Id, subject_id: Subject::Id) -> Self
    where
        Target: Aggregate,
        Subject: Aggregate,
    {
        Self::from_parts(
            AggregateRef::from_id::<Target>(target_id),
            RelationshipSubject::aggregate::<Subject>(subject_id),
        )
    }

    pub fn from_parts(target: AggregateRef, subject: RelationshipSubject) -> Self {
        Self { target, subject }
    }
}
