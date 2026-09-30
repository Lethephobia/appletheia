use appletheia::saga_step;

#[saga_step]
pub enum OwnedAccountClosureSagaStep {
    Request,
    Advance,
    ProcessPage,
    RecordClosedAccount,
    RecordRejectedAccountClose,
    Fail,
}
