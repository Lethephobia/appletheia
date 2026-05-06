#[derive(Clone, Debug, PartialEq)]
pub enum SagaStatus {
    InProgress,
    Succeeded,
    Failed,
}
