use appletheia_domain::Aggregate;

use super::RelationshipEntry;

/// A deterministic, deduplicated set returned by one source declaration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RelationshipEntries(Vec<RelationshipEntry>);

impl RelationshipEntries {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<Target, Subject>(
        &mut self,
        target_id: Target::Id,
        subject_id: Subject::Id,
    ) -> bool
    where
        Target: Aggregate,
        Subject: Aggregate,
    {
        self.insert_entry(RelationshipEntry::new::<Target, Subject>(
            target_id, subject_id,
        ))
    }

    pub fn insert_entry(&mut self, entry: RelationshipEntry) -> bool {
        if self.0.contains(&entry) {
            return false;
        }
        self.0.push(entry);
        true
    }

    pub fn iter(&self) -> impl Iterator<Item = &RelationshipEntry> {
        self.0.iter()
    }
}
