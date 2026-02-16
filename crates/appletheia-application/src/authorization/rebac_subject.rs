use crate::request_context::SubjectRef;

use super::{RelationName, ResourceRef};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RebacSubject {
    Subject(SubjectRef),
    SubjectSet { object: ResourceRef, relation: RelationName },
}

