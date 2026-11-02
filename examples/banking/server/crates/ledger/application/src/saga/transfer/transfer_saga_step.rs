use appletheia::saga_step;

/// Lists the logical command-dispatch steps in a transfer saga.
#[saga_step]
pub enum TransferSagaStep {
    ReserveFunds,
    Deposit,
    ReleaseFunds,
    CommitFunds,
    CompensateDeposit,
    Succeed,
    Fail,
}
