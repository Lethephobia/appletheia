use banking_ledger_domain::core::OnchainTransactionId;

/// Contains the canonical transaction identity returned by withdrawal execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawalSettlementExecution {
    pub transaction_id: OnchainTransactionId,
}
