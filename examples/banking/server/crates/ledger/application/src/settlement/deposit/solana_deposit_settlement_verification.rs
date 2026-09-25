use banking_ledger_domain::core::SolanaTransactionSignature;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaDepositSettlementVerification {
    pub transaction_id: SolanaTransactionSignature,
}
