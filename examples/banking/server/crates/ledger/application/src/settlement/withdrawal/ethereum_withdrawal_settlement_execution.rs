use banking_ledger_domain::core::EvmTransactionHash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumWithdrawalSettlementExecution {
    pub transaction_id: EvmTransactionHash,
}
