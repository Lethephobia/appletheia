use super::{Caveat, RebacSubject};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipEdge {
    pub subject: RebacSubject,
    pub caveat: Option<Caveat>,
}

