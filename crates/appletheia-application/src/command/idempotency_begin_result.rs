use super::IdempotencyOutput;

#[derive(Clone, Debug, PartialEq)]
pub enum IdempotencyBeginResult {
    New,
    Existing { output: IdempotencyOutput },
    InProgress,
}
