use appletheia::saga_step;

/// Lists the logical command-dispatch steps in a deposit saga.
#[saga_step]
pub enum DepositSagaStep {
    Deposit,
    Succeed,
    Fail,
}
