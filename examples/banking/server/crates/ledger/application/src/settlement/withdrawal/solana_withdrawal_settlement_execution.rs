use banking_ledger_domain::onchain::SolanaTransactionSignature;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaWithdrawalSettlementExecution {
    pub transaction_id: SolanaTransactionSignature,
}
