use banking_ledger_domain::core::EvmTransactionHash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumDepositSettlementVerification {
    pub transaction_id: EvmTransactionHash,
}
