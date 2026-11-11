use banking_ledger_domain::onchain::SolanaTransactionSignature;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaDepositSettlementVerification {
    pub transaction_id: SolanaTransactionSignature,
}
