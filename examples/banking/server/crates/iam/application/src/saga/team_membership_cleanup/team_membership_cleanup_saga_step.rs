use appletheia::saga_step;

#[saga_step]
pub enum TeamMembershipCleanupSagaStep {
    Start,
    ProcessPage,
    RecordMembershipSucceeded,
    RecordMembershipFailed,
}
