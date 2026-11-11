use banking_ledger_domain::onchain::EvmTransactionHash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumWithdrawalSettlementExecution {
    pub transaction_id: EvmTransactionHash,
}
