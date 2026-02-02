use super::IdempotencyState;

#[derive(Clone, Debug, PartialEq)]
pub enum IdempotencyBeginResult {
    New,
    Existing { state: IdempotencyState },
    InProgress,
}
