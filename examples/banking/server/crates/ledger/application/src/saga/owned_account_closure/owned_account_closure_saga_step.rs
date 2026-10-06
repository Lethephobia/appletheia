use appletheia::saga_step;

#[saga_step]
pub enum OwnedAccountClosureSagaStep {
    Start,
    ProcessPage,
    RecordSucceeded,
    RecordFailed,
}
