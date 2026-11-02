use banking_ledger_domain::core::OnchainTransactionId;

/// Contains the canonical transaction identity returned by deposit verification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositSettlementVerification {
    pub transaction_id: OnchainTransactionId,
}
