#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UsersetExprEvalDepth(usize);

impl UsersetExprEvalDepth {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }

    pub fn increment(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}
