#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RelationshipEvalNodeCount(usize);

impl RelationshipEvalNodeCount {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }

    pub fn saturating_add(self, rhs: usize) -> Self {
        Self(self.0.saturating_add(rhs))
    }
}
