use super::Relationship;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RelationshipChange {
    Upsert(Relationship),
    Delete(Relationship),
}
