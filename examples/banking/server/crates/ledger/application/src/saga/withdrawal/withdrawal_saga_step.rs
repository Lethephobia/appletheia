use appletheia::saga_step;

/// Lists the logical command-dispatch steps in a withdrawal saga.
#[saga_step]
pub enum WithdrawalSagaStep {
    ReserveFunds,
    ExecuteSettlement,
    ReleaseFunds,
    CommitFunds,
    Succeed,
    Fail,
}
